/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::cickinds::CICKind;
use crate::{detect, error::Ipl3ChecksumError, utils};

fn read_word_from_ram(rom_words: &[u32], entrypoint_ram: u32, ram_addr: u32) -> u32 {
    rom_words[((ram_addr - entrypoint_ram + 0x1000) / 4) as usize]
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
    if rom_bytes.len() < 0x101000 {
        return Err(Ipl3ChecksumError::BufferNotBigEnough {
            buffer_len: rom_bytes.len(),
            expected_len: 0x101000,
        });
    }

    let rom_words = utils::read_u32_vec(rom_bytes, 0, 0x101000 / 4)?;

    let seed = kind.get_seed();
    let magic = kind.get_magic();

    let mut s6 = seed;

    let mut a0 = rom_words[8 / 4];
    if kind == CICKind::CIC_X103 {
        a0 = a0.wrapping_sub(0x100000);
    }
    if kind == CICKind::CIC_X106 {
        a0 = a0.wrapping_sub(0x200000);
    }
    let entrypoint_ram = a0;

    let mut at = magic;
    let lo = s6.wrapping_mul(at);

    if kind == CICKind::CIC_X105 {
        s6 = 0xA0000200;
    }

    let ra = 0x100000;

    let mut t0: u32 = 0;

    let mut t1: u32 = a0;

    let t5: u32 = 0x20;

    //let mut v0 = utils.u32(lo);
    let mut v0 = lo;
    v0 += 1;

    let mut a3 = v0;
    let mut t2 = v0;
    let mut t3 = v0;
    let mut s0 = v0;
    let mut a2 = v0;
    let mut t4 = v0;

    // poor man's do while
    // LA40005F0_loop
    let mut loop_variable = true;
    while loop_variable {
        // v0 = *t1
        v0 = read_word_from_ram(&rom_words, entrypoint_ram, t1);

        //v1 = utils.u32(a3 + v0);
        let mut v1 = a3.wrapping_add(v0);

        //at = utils.u32(v1) < utils.u32(a3);
        at = if v1 < a3 { 1 } else { 0 };

        let a1 = v1;
        // if (at == 0) goto LA4000608;

        if at != 0 {
            //t2 = utils.u32(t2 + 0x1)
            t2 = t2.wrapping_add(0x1);
        }

        // LA4000608
        v1 = v0 & 0x1F;
        //t7 = utils.u32(t5 - v1)
        let t7: u32 = t5.wrapping_sub(v1);

        //let t8 = utils.u32(v0 >> t7)
        //let t6 = utils.u32(v0 << v1)
        let t8 = v0.wrapping_shr(t7);
        let t6 = v0.wrapping_shl(v1);

        a0 = t6 | t8;
        // at = utils.u32(a2) < utils.u32(v0);
        at = if a2 < v0 { 1 } else { 0 };
        a3 = a1;

        t3 ^= v0;

        //s0 = utils.u32(s0 + a0)
        s0 = s0.wrapping_add(a0);
        // if (at == 0) goto LA400063C;
        if at != 0 {
            let t9 = a3 ^ v0;

            a2 ^= t9;
            // goto LA4000640;

            // LA400063C:
        } else {
            a2 ^= a0;
        }

        // LA4000640:
        if kind == CICKind::CIC_X105 {
            // ipl3 6105 copies 0x330 bytes from the ROM's offset 0x000554 (or offset 0x000514 into IPL3) to vram 0xA0000004
            let mut t7 = rom_words[((s6 - 0xA0000004 + 0x000554) / 4) as usize];

            //t0 = utils.u32(t0 + 0x4);
            //s6 = utils.u32(s6 + 0x4);
            t0 = t0.wrapping_add(0x4);
            s6 = s6.wrapping_add(0x4);

            t7 ^= v0;

            // t4 = utils.u32(t7 + t4);
            t4 = t7.wrapping_add(t4);

            t7 = 0xA00002FF;

            // t1 = utils.u32(t1 + 0x4);
            t1 = t1.wrapping_add(0x4);

            // s6 = utils.u32(s6 & t7);
            s6 &= t7;
        } else {
            // t0 = utils.u32(t0 + 0x4);
            t0 = t0.wrapping_add(0x4);

            let t7 = v0 ^ s0;

            // t1 = utils.u32(t1 + 0x4);
            t1 = t1.wrapping_add(0x4);

            // t4 = utils.u32(t7 + t4);
            t4 = t7.wrapping_add(t4);
        }

        // if (t0 != ra) goto LA40005F0;
        if t0 == ra {
            loop_variable = false;
        }
    }

    if kind == CICKind::CIC_X103 {
        let t6 = a3 ^ t2;
        // a3 = utils.u32(t6 + t3);
        a3 = t6.wrapping_add(t3);

        let t8 = s0 ^ a2;
        // s0 = utils.u32(t8 + t4);
        s0 = t8.wrapping_add(t4);
    } else if kind == CICKind::CIC_X106 {
        /*
        let t6 = utils.u32(a3 * t2);
        a3 = utils.u32(t6 + t3);
        let t8 = utils.u32(s0 * a2);
        s0 = utils.u32(t8 + t4);
        */
        let t6 = a3.wrapping_mul(t2);
        a3 = t6.wrapping_add(t3);
        let t8 = s0.wrapping_mul(a2);
        s0 = t8.wrapping_add(t4);
    } else {
        let t6 = a3 ^ t2;
        a3 = t6 ^ t3;
        let t8 = s0 ^ a2;
        s0 = t8 ^ t4;
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

    #[pyfunction]
    pub(crate) fn calculateChecksum(
        rom_bytes: &[u8],
        kind: super::CICKind,
    ) -> Result<Option<(u32, u32)>, super::Ipl3ChecksumError> {
        match super::calculate_checksum(rom_bytes, kind) {
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
        rom_bytes: &[u8],
    ) -> Result<Option<(u32, u32)>, super::Ipl3ChecksumError> {
        match super::calculate_checksum_autodetect(rom_bytes) {
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

        let bytes = match super::utils::c_bindings::u8_vec_from_pointer_array(rom_bytes_len, rom_bytes) {
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

        let bytes = match super::utils::c_bindings::u8_vec_from_pointer_array(rom_bytes_len, rom_bytes) {
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
