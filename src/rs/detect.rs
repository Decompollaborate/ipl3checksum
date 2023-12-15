/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::cickinds::CICKind;
use crate::utils;

pub fn detect_cic_raw(raw_bytes: &[u8]) -> Option<CICKind> {
    /*Tries to detect an IPL3 binary.

    The argument to this function must be exactly the IPL3 binary, stripping the rest of the ROM.

    Args:
        raw_bytes (bytes): IPL3 binary in big endian format.

    Returns:
        CICKind|None: The detected CIC kind, or `None` if was not able to detect the CIC kind.
    */

    if raw_bytes.len() != 0xFC0 {
        return None;
    }

    let bytes_hash = utils::get_hash_md5(raw_bytes);

    CICKind::from_hash_md5(&bytes_hash)
}

pub fn detect_cic(rom_bytes: &[u8]) -> Option<CICKind> {
    /*Tries to detect an IPL3 in a ROM.

    The argument to this function must be a ROM in big endian format.

    Args:
        rom_bytes (bytes): ROMbinary in big endian format.

    Returns:
        CICKind|None: The detected CIC kind, or `None` if was not able to detect the CIC kind.
    */

    detect_cic_raw(&rom_bytes[0x40..0x1000])
}

#[cfg(feature = "python_bindings")]
#[allow(non_snake_case)]
pub(crate) mod python_bindings {
    use pyo3::prelude::*;

    #[pyfunction]
    pub(crate) fn detectCICRaw(raw_bytes: &[u8]) -> Option<super::CICKind> {
        super::detect_cic_raw(raw_bytes)
    }

    #[pyfunction]
    pub(crate) fn detectCIC(rom_bytes: &[u8]) -> Option<super::CICKind> {
        super::detect_cic(rom_bytes)
    }
}
