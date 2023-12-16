/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;

/* This needs to be in sync with the C equivalent at `bindings/c/include/ipl3checksum/cickinds.h` */
#[cfg_attr(feature = "python_bindings", pyclass(module = "ipl3checksum"))]
// repr is kinda complex and I may have got it wrong.
// I tried to follow the stuff at https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html
#[cfg_attr(feature = "c_bindings", repr(C))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
/// Enum that represents a CIC kind
pub enum CICKind {
    CIC_6101,
    CIC_6102_7101,
    CIC_7102,
    CIC_X103, // Both 6103 and 7103
    // 6104/7104 does not exist
    CIC_X105, // Both 6105 and 7105
    CIC_X106, // Both 6106 and 7106
}

impl CICKind {
    pub fn get_seed(&self) -> u32 {
        match self {
            Self::CIC_6101 | Self::CIC_6102_7101 | Self::CIC_7102 => 0x3F,
            Self::CIC_X103 => 0x78,
            Self::CIC_X105 => 0x91,
            Self::CIC_X106 => 0x85,
        }
    }

    pub fn get_magic(&self) -> u32 {
        match self {
            Self::CIC_6101 | Self::CIC_6102_7101 | Self::CIC_7102 | Self::CIC_X105 => 0x5D588B65,
            Self::CIC_X103 | Self::CIC_X106 => 0x6C078965,
        }
    }

    pub fn get_hash_md5(&self) -> &str {
        match self {
            Self::CIC_6101 => "900b4a5b68edb71f4c7ed52acd814fc5",
            Self::CIC_6102_7101 => "e24dd796b2fa16511521139d28c8356b",
            Self::CIC_7102 => "955894c2e40a698bf98a67b78a4e28fa",
            Self::CIC_X103 => "319038097346e12c26c3c21b56f86f23",
            Self::CIC_X105 => "ff22a296e55d34ab0a077dc2ba5f5796",
            Self::CIC_X106 => "6460387749ac0bd925aa5430bc7864fe",
        }
    }

    pub fn from_hash_md5(hash_str: &str) -> Option<CICKind> {
        match hash_str {
            "900b4a5b68edb71f4c7ed52acd814fc5" => Some(Self::CIC_6101),
            "e24dd796b2fa16511521139d28c8356b" => Some(Self::CIC_6102_7101),
            "955894c2e40a698bf98a67b78a4e28fa" => Some(Self::CIC_7102),
            "319038097346e12c26c3c21b56f86f23" => Some(Self::CIC_X103),
            "ff22a296e55d34ab0a077dc2ba5f5796" => Some(Self::CIC_X105),
            "6460387749ac0bd925aa5430bc7864fe" => Some(Self::CIC_X106),
            _ => None,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Self::CIC_6101 => "CIC_6101",
            Self::CIC_6102_7101 => "CIC_6102_7101",
            Self::CIC_7102 => "CIC_7102",
            Self::CIC_X103 => "CIC_X103",
            Self::CIC_X105 => "CIC_X105",
            Self::CIC_X106 => "CIC_X106",
        }
    }

    pub fn from_name(name: &str) -> Option<CICKind> {
        match name {
            "CIC_6101" | "6101" => Some(Self::CIC_6101),
            "CIC_6102_7101" | "CIC_6102" | "CIC_7101" | "6102_7101" | "6102" | "7101" => {
                Some(Self::CIC_6102_7101)
            }
            "CIC_7102" | "7102" => Some(Self::CIC_7102),
            "CIC_X103" | "CIC_6103" | "CIC_7103" | "X103" | "6103" | "7103" => Some(Self::CIC_X103),
            "CIC_X105" | "CIC_6105" | "CIC_7105" | "X105" | "6105" | "7105" => Some(Self::CIC_X105),
            "CIC_X106" | "CIC_6106" | "CIC_7106" | "X106" | "6106" | "7106" => Some(Self::CIC_X106),
            _ => None,
        }
    }

    pub fn from_value(value: usize) -> Option<CICKind> {
        match value {
            6101 => Some(Self::CIC_6101),
            6102 | 7101 => Some(Self::CIC_6102_7101),
            7102 => Some(Self::CIC_7102),
            6103 | 7103 => Some(Self::CIC_X103),
            6105 | 7105 => Some(Self::CIC_X105),
            6106 | 7106 => Some(Self::CIC_X106),
            _ => None,
        }
    }
}

#[cfg(feature = "python_bindings")]
#[allow(non_snake_case)]
mod python_bindings {
    use pyo3::prelude::*;

    #[pymethods]
    impl super::CICKind {
        pub fn getSeed(&self) -> u32 {
            self.get_seed()
        }

        pub fn getMagic(&self) -> u32 {
            self.get_magic()
        }

        pub fn getHashMd5(&self) -> &str {
            self.get_hash_md5()
        }

        #[staticmethod]
        pub fn fromHashMd5(hash_str: &str) -> Option<Self> {
            Self::from_hash_md5(hash_str)
        }

        #[getter]
        pub fn name(&self) -> &str {
            self.get_name()
        }

        #[staticmethod]
        pub fn fromName(name: &str) -> Option<Self> {
            Self::from_name(name)
        }

        #[staticmethod]
        pub fn fromValue(value: usize) -> Option<Self> {
            Self::from_value(value)
        }
    }
}

#[cfg(feature = "c_bindings")]
mod c_bindings {
    use crate::{CICKind, Ipl3ChecksumError, utils};

    #[no_mangle]
    pub extern "C" fn ipl3checksum_cickind_get_seed(
        kind: CICKind
    ) -> u32 {
        kind.get_seed()
    }

    #[no_mangle]
    pub extern "C" fn ipl3checksum_cickind_get_magic(
        kind: CICKind
    ) -> u32 {
        kind.get_magic()
    }

    /*
    #[no_mangle]
    pub extern "C" fn ipl3checksum_cickind_get_hash_md5(
        kind: CICKind
    ) -> *const core::ffi::c_char {
        std::ffi::CString::new(kind.get_hash_md5()).unwrap().as_ptr()
    }
    */

    #[no_mangle]
    pub extern "C" fn ipl3checksum_cickind_from_hash_md5(
        kind_dst: *mut CICKind,
        hash_str: *const core::ffi::c_char
    ) -> Ipl3ChecksumError {
        if kind_dst.is_null() || hash_str.is_null() {
            return Ipl3ChecksumError::NullPointer;
        }

        let hash = match utils::static_str_from_c_string(hash_str) {
            Err(e) => return e,
            Ok(h) => h,
        };

        let kind = match CICKind::from_hash_md5(hash) {
            Some(k) => k,
            None => return Ipl3ChecksumError::UnableToDetectCIC,
        };

        unsafe { *kind_dst = kind };

        Ipl3ChecksumError::Okay
    }

    /*
    #[no_mangle]
    pub extern "C" fn ipl3checksum_cickind_get_name(
        kind: CICKind
    ) ->  {
    }
    */

    #[no_mangle]
    pub extern "C" fn ipl3checksum_cickind_from_name(
        kind_dst: *mut CICKind,
        c_name: *const core::ffi::c_char
    ) -> Ipl3ChecksumError {
        if kind_dst.is_null() || c_name.is_null() {
            return Ipl3ChecksumError::NullPointer;
        }

        let name = match utils::static_str_from_c_string(c_name) {
            Err(e) => return e,
            Ok(h) => h,
        };

        let kind = match CICKind::from_name(name) {
            Some(k) => k,
            None => return Ipl3ChecksumError::UnableToDetectCIC,
        };

        unsafe { *kind_dst = kind };

        Ipl3ChecksumError::Okay
    }

    #[no_mangle]
    pub extern "C" fn ipl3checksum_cickind_from_value(
        kind_dst: *mut CICKind,
        value: usize
    ) -> Ipl3ChecksumError {
        if kind_dst.is_null() {
            return Ipl3ChecksumError::NullPointer;
        }

        let kind = match CICKind::from_value(value) {
            Some(k) => k,
            None => return Ipl3ChecksumError::UnableToDetectCIC,
        };

        unsafe { *kind_dst = kind };

        Ipl3ChecksumError::Okay
    }
}
