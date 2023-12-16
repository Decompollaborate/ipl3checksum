/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{cickinds::CICKind, error::Ipl3ChecksumError, utils};

/// Tries to detect an IPL3 binary.
///
/// The argument to this function must be exactly the IPL3 binary.
///
/// ## Arguments
///
/// * `raw_bytes` - IPL3 binary in big endian format.
///
/// ## Return
///
/// * The detected CIC kind, or `Ipl3ChecksumError` if was not able to detect the CIC kind.
pub fn detect_cic_raw(raw_bytes: &[u8]) -> Result<CICKind, Ipl3ChecksumError> {
    if raw_bytes.len() != 0xFC0 {
        return Err(Ipl3ChecksumError::BufferSizeIsWrong{buffer_len:raw_bytes.len() , expected_len: 0xFC0});
    }

    let bytes_hash = utils::get_hash_md5(raw_bytes);

    match CICKind::from_hash_md5(&bytes_hash) {
        Some(cic) => Ok(cic),
        None => Err(Ipl3ChecksumError::UnableToDetectCIC{hash: bytes_hash}),
    }
}

/// Tries to detect an IPL3 in a ROM.
///
/// The argument to this function must be a ROM in big endian format.
///
/// ## Arguments
///
/// * `rom_bytes` - ROM binary in big endian format.
///
/// ## Return
///
/// * The detected CIC kind, or `None` if was not able to detect the CIC kind.
pub fn detect_cic(rom_bytes: &[u8]) -> Result<CICKind, Ipl3ChecksumError> {
    detect_cic_raw(&rom_bytes[0x40..0x1000])
}

#[cfg(feature = "python_bindings")]
#[allow(non_snake_case)]
pub(crate) mod python_bindings {
    use pyo3::prelude::*;

    #[pyfunction]
    pub(crate) fn detectCICRaw(raw_bytes: &[u8]) -> Result<Option<super::CICKind>, super::Ipl3ChecksumError> {
        match super::detect_cic_raw(raw_bytes) {
            Ok(cic) => Ok(Some(cic)),
            Err(e) => match e {
                super::Ipl3ChecksumError::BufferSizeIsWrong { buffer_len, expected_len } => Ok(None),
                super::Ipl3ChecksumError::UnableToDetectCIC { hash } => Ok(None),
                _ => Err(e), // To trigger an exception on Python's side
            },
        }
    }

    #[pyfunction]
    pub(crate) fn detectCIC(rom_bytes: &[u8]) -> Result<Option<super::CICKind>, super::Ipl3ChecksumError> {
        match super::detect_cic(rom_bytes) {
            Ok(cic) => Ok(Some(cic)),
            Err(e) => match e {
                super::Ipl3ChecksumError::BufferSizeIsWrong { buffer_len, expected_len } => Ok(None),
                super::Ipl3ChecksumError::UnableToDetectCIC { hash } => Ok(None),
                _ => Err(e), // To trigger an exception on Python's side
            },
        }
    }
}
