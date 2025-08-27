/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::cickinds::CICKind;
use crate::{detect, error::Ipl3ChecksumError, utils};

const HEADER_IPL3_SIZE: usize = 0x1000;
const WORD_SIZE: usize = 4;

fn get_entrypoint_addr(rom_bytes: &[u8], kind: CICKind) -> Result<u32, Ipl3ChecksumError> {
    let entrypoint_addr: u32 = utils::read_u32(rom_bytes, 2 * WORD_SIZE)?;

    Ok(kind.get_entrypoint(entrypoint_addr))
}

/// Calculates the checksum required by an official CIC of a N64 ROM.
///
/// ## Arguments
///
/// * `rom_bytes` - The bytes of the N64 ROM in big endian format. It must have a minimum size of 0x101000 bytes.
/// * `kind` - The CIC kind variation used to calculate the checksum.
///
/// ## Return
///
/// * If no error happens then the calculated checksum is returned, stored as a tuple
///   containing two 32-bits words.
///
/// ## Examples
///
/// ```
/// use ipl3checksum;
/// let bytes = vec![0; 0x101000];
/// let kind = ipl3checksum::CICKind::CIC_6102_7101;
/// let checksum = ipl3checksum::calculate_checksum(&bytes, kind).unwrap();
/// println!("{:08X} {:08X}", checksum.0, checksum.1);
/// ```
pub fn calculate_checksum(
    rom_bytes: &[u8],
    kind: CICKind,
) -> Result<(u32, u32), Ipl3ChecksumError> {
    let seed = kind.get_seed();
    let magic = kind.get_magic();

    let v0 = seed.wrapping_mul(magic).wrapping_add(1);

    let mut a3 = v0;
    let mut t2 = v0;
    let mut t3 = v0;
    let mut s0 = v0;
    let mut a2 = v0;
    let mut t4 = v0;

    // Get how many bytes of the ROM (passed IPL3) to check
    let bytes_to_check: u32 =
        // IPL3 5101 checks almost 4 times the normal amount depending on the entrypoint
        if (kind == CICKind::CIC_5101) && (get_entrypoint_addr(rom_bytes, kind)? == 0x80000400) {
            0x3FE000 // ~ 3.992 MiB
        } else {
            0x100000
        };

    // Error if the ROM is not big enough
    if rom_bytes.len() < bytes_to_check as usize + HEADER_IPL3_SIZE {
        return Err(Ipl3ChecksumError::BufferNotBigEnough {
            buffer_len: rom_bytes.len(),
            expected_len: bytes_to_check as usize + HEADER_IPL3_SIZE,
        });
    }

    let mut rom_words_iter = rom_bytes
        .chunks_exact(WORD_SIZE)
        .skip(HEADER_IPL3_SIZE / WORD_SIZE)
        .take((bytes_to_check as usize + HEADER_IPL3_SIZE) / WORD_SIZE)
        .map(|arr| {
            // Unwraping should be fine since we are requesting for chunks of this exact size.
            u32::from_be_bytes(arr.try_into().unwrap())
        });

    let words_to_check = bytes_to_check.wrapping_div(WORD_SIZE as u32) as usize;
    for i in 0..words_to_check {
        let word = rom_words_iter.next().unwrap();

        let a1 = a3.wrapping_add(word);
        if a1 < a3 {
            t2 = t2.wrapping_add(0x1);
        }
        a3 = a1;

        let a0 = word.rotate_left(word & 0x1F);

        t3 ^= word;

        s0 = s0.wrapping_add(a0);
        if a2 < word {
            a2 ^= a3 ^ word;
        } else {
            a2 ^= a0;
        }

        if kind == CICKind::CIC_X105 {
            // ipl3 6105 copies 0x330 bytes from the ROM's offset 0x000554 (or offset 0x000514 into IPL3) to vram 0xA0000004
            let temp = (i & 0x3F) | 0x80;
            let displacement = &rom_bytes[(temp + 0x154) * WORD_SIZE..];
            let t7 = u32::from_be_bytes([
                displacement[0],
                displacement[1],
                displacement[2],
                displacement[3],
            ]);

            t4 = t4.wrapping_add(word ^ t7);
        } else {
            t4 = t4.wrapping_add(word ^ s0);
        }
    }

    match kind {
        CICKind::CIC_X103 | CICKind::CIC_5101 => {
            let t6 = a3 ^ t2;
            a3 = t6.wrapping_add(t3);

            let t8 = s0 ^ a2;
            s0 = t8.wrapping_add(t4);
        }
        CICKind::CIC_X106 => {
            let t6 = a3.wrapping_mul(t2);
            a3 = t6.wrapping_add(t3);

            let t8 = s0.wrapping_mul(a2);
            s0 = t8.wrapping_add(t4);
        }
        _ => {
            let t6 = a3 ^ t2;
            a3 = t6 ^ t3;

            let t8 = s0 ^ a2;
            s0 = t8 ^ t4;
        }
    }

    Ok((a3, s0))
}

/// Calculates the checksum required by an official CIC of a N64 ROM.
///
/// This function will try to autodetect the CIC kind automatically.
/// If it fails to detect it then an error will be returned.
///
/// ## Arguments
///
/// * `rom_bytes` - The bytes of the N64 ROM in big endian format. It must have a minimum size of 0x101000 bytes.
///
/// ## Return
///
/// * If no error happens then the calculated checksum is returned, stored as a tuple
///   containing two 32-bits words. Otherwise, `None` is returned.
///   Possible errors:
///     - `rom_bytes` not being big enough
///     - Not able to detect the CIC kind
///
/// ## Examples
///
/// ```
/// use ipl3checksum;
/// let bytes = vec![0; 0x101000];
/// let checksum = ipl3checksum::calculate_checksum_autodetect(&bytes);
/// /* This will return `None` because there's no ipl3 binary on an array of zeroes */
/// assert!(checksum.is_err());
/// ```
pub fn calculate_checksum_autodetect(rom_bytes: &[u8]) -> Result<(u32, u32), Ipl3ChecksumError> {
    let kind = detect::detect_cic(rom_bytes)?;

    calculate_checksum(rom_bytes, kind)
}

#[cfg(feature = "python_bindings")]
#[allow(non_snake_case)]
pub(crate) mod python_bindings {
    use pyo3::prelude::*;
    use std::borrow::Cow;

    /**
     * We use a `Cow` instead of a plain &[u8] the latter only allows Python's
     * `bytes` objects, while Cow allows for both `bytes` and `bytearray`.
     * This is important because an argument typed as `bytes` allows to pass a
     * `bytearray` object too.
     */

    #[pyfunction]
    pub(crate) fn calculateChecksum(
        rom_bytes: Cow<[u8]>,
        kind: super::CICKind,
    ) -> Result<Option<(u32, u32)>, super::Ipl3ChecksumError> {
        match super::calculate_checksum(&rom_bytes, kind) {
            Ok(checksum) => Ok(Some(checksum)),
            Err(e) => match e {
                super::Ipl3ChecksumError::BufferNotBigEnough {
                    buffer_len: _,
                    expected_len: _,
                } => Ok(None),
                _ => Err(e), // To trigger an exception on Python's side
            },
        }
    }

    #[pyfunction]
    pub(crate) fn calculateChecksumAutodetect(
        rom_bytes: Cow<[u8]>,
    ) -> Result<Option<(u32, u32)>, super::Ipl3ChecksumError> {
        match super::calculate_checksum_autodetect(&rom_bytes) {
            Ok(checksum) => Ok(Some(checksum)),
            Err(e) => match e {
                super::Ipl3ChecksumError::BufferNotBigEnough {
                    buffer_len: _,
                    expected_len: _,
                } => Ok(None),
                _ => Err(e), // To trigger an exception on Python's side
            },
        }
    }
}

#[cfg(feature = "c_bindings")]
mod c_bindings {
    #[no_mangle]
    pub extern "C" fn ipl3checksum_calculate_checksum(
        dst_checksum0: *mut u32,
        dst_checksum1: *mut u32,
        rom_bytes_len: usize,
        rom_bytes: *const u8,
        kind: super::CICKind,
    ) -> super::Ipl3ChecksumError {
        if dst_checksum0.is_null() || dst_checksum1.is_null() || rom_bytes.is_null() {
            return super::Ipl3ChecksumError::NullPointer;
        }

        let bytes =
            match super::utils::c_bindings::u8_vec_from_pointer_array(rom_bytes_len, rom_bytes) {
                Err(e) => return e,
                Ok(d) => d,
            };

        let checksum = match super::calculate_checksum(&bytes, kind) {
            Ok(chk) => chk,
            Err(e) => return e,
        };

        unsafe { *dst_checksum0 = checksum.0 };
        unsafe { *dst_checksum1 = checksum.1 };

        super::Ipl3ChecksumError::Okay
    }

    #[no_mangle]
    pub extern "C" fn ipl3checksum_calculate_checksum_autodetect(
        dst_checksum0: *mut u32,
        dst_checksum1: *mut u32,
        rom_bytes_len: usize,
        rom_bytes: *const u8,
    ) -> super::Ipl3ChecksumError {
        if dst_checksum0.is_null() || dst_checksum1.is_null() || rom_bytes.is_null() {
            return super::Ipl3ChecksumError::NullPointer;
        }

        let bytes =
            match super::utils::c_bindings::u8_vec_from_pointer_array(rom_bytes_len, rom_bytes) {
                Err(e) => return e,
                Ok(d) => d,
            };

        let checksum = match super::calculate_checksum_autodetect(&bytes) {
            Ok(chk) => chk,
            Err(e) => return e,
        };

        unsafe { *dst_checksum0 = checksum.0 };
        unsafe { *dst_checksum1 = checksum.1 };

        super::Ipl3ChecksumError::Okay
    }
}
