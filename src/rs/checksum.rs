/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::cickinds::CICKind;
use crate::{detect, error::Ipl3ChecksumError, utils};

fn get_entrypoint_addr(rom_bytes: &[u8], kind: CICKind) -> Result<u32, Ipl3ChecksumError> {
    let entrypoint_addr: u32 = utils::read_u32(rom_bytes, 8)?;

    match kind {
        CICKind::CIC_X103 | CICKind::CIC_5101 => Ok(entrypoint_addr.wrapping_sub(0x100000)),
        CICKind::CIC_X106 => Ok(entrypoint_addr.wrapping_sub(0x200000)),
        _ => Ok(entrypoint_addr),
    }
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
///   containing two 32-bits words. Otherwise, `None` is returned.
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

    let ra: u32 =
        if (kind == CICKind::CIC_5101) && (get_entrypoint_addr(rom_bytes, kind)? == 0x80000400) {
            0x3FE000
        } else {
            0x100000
        };

    if rom_bytes.len() < ra as usize + 0x1000 {
        return Err(Ipl3ChecksumError::BufferNotBigEnough {
            buffer_len: rom_bytes.len(),
            expected_len: ra as usize + 0x1000,
        });
    }

    let rom_words = utils::read_u32_vec(rom_bytes, 0, (ra as usize + 0x1000) / 4)?;

    let mut t0: u32 = 0;
    while t0 < ra {
        let v0 = rom_words[((t0 + 0x1000) / 4) as usize];

        let mut v1 = a3.wrapping_add(v0);

        let a1 = v1;

        if v1 < a3 {
            t2 = t2.wrapping_add(0x1);
        }

        v1 = v0 & 0x1F;
        let t7: u32 = 0x20_u32.wrapping_sub(v1);

        let t8 = v0.wrapping_shr(t7);
        let t6 = v0.wrapping_shl(v1);

        let a0 = t6 | t8;
        a3 = a1;

        t3 ^= v0;

        s0 = s0.wrapping_add(a0);
        if a2 < v0 {
            let t9 = a3 ^ v0;

            a2 ^= t9;
        } else {
            a2 ^= a0;
        }

        if kind == CICKind::CIC_X105 {
            // ipl3 6105 copies 0x330 bytes from the ROM's offset 0x000554 (or offset 0x000514 into IPL3) to vram 0xA0000004
            let temp: u32 = (t0 & 0xFF) | 0x200;
            let t7 = rom_words[((temp - 0x4 + 0x000554) / 4) as usize];

            t4 = t4.wrapping_add(v0 ^ t7);
        } else {
            t4 = t4.wrapping_add(v0 ^ s0);
        }

        t0 = t0.wrapping_add(0x4);
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

#[cfg(test)]
mod tests {
    use crate::{cickinds::CICKind, utils};
    use std::fs;

    #[test]
    fn test_dummy_files() -> Result<(), super::Ipl3ChecksumError> {
        for path_result in fs::read_dir("tests/dummytests").unwrap() {
            let ipl3_folder = path_result.unwrap();
            let folder_name = ipl3_folder.file_name();

            println!("{:?}", folder_name);

            let kind = CICKind::from_name(folder_name.to_str().unwrap()).unwrap();
            println!("CIC Kind: {:?}", kind);

            for bin_path_result in fs::read_dir(ipl3_folder.path()).unwrap() {
                let bin_path = bin_path_result.unwrap();

                println!("{:?}", bin_path);

                println!("    Reading...");

                let bin_bytes = fs::read(&bin_path.path()).unwrap();

                println!("    Calculating checksum...");
                let checksum = super::calculate_checksum(&bin_bytes, kind).unwrap();
                println!("Used CIC Kind: {:?}", kind);

                println!(
                    "    Calculated checksum is: 0x{:08X} 0x{:08X}",
                    checksum.0, checksum.1
                );

                println!("    Checking checksum...");
                let bin_checksum = utils::read_u32_vec(&bin_bytes, 0x10, 2)?;

                println!(
                    "    Expected checksum is: 0x{:08X} 0x{:08X}",
                    bin_checksum[0], bin_checksum[1]
                );

                assert_eq!(checksum.0, bin_checksum[0]);
                assert_eq!(checksum.1, bin_checksum[1]);

                println!("    {:?} OK", bin_path);

                println!();
            }

            println!();
        }
        Ok(())
    }
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
