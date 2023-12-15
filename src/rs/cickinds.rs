/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;

#[cfg_attr(feature = "python_bindings", pyclass(module = "ipl3checksum"))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
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
            CICKind::CIC_6101 => 0x3F,
            CICKind::CIC_6102_7101 => 0x3F,
            CICKind::CIC_7102 => 0x3F,
            CICKind::CIC_X103 => 0x78,
            CICKind::CIC_X105 => 0x91,
            CICKind::CIC_X106 => 0x85,
        }
    }

    pub fn get_magic(&self) -> u32 {
        match self {
            CICKind::CIC_6101 => 0x5D588B65,
            CICKind::CIC_6102_7101 => 0x5D588B65,
            CICKind::CIC_7102 => 0x5D588B65,
            CICKind::CIC_X103 => 0x6C078965,
            CICKind::CIC_X105 => 0x5D588B65,
            CICKind::CIC_X106 => 0x6C078965,
        }
    }

    pub fn get_hash_md5(&self) -> &str {
        match self {
            CICKind::CIC_6101 => "900b4a5b68edb71f4c7ed52acd814fc5",
            CICKind::CIC_6102_7101 => "e24dd796b2fa16511521139d28c8356b",
            CICKind::CIC_7102 => "955894c2e40a698bf98a67b78a4e28fa",
            CICKind::CIC_X103 => "319038097346e12c26c3c21b56f86f23",
            CICKind::CIC_X105 => "ff22a296e55d34ab0a077dc2ba5f5796",
            CICKind::CIC_X106 => "6460387749ac0bd925aa5430bc7864fe",
        }
    }

    pub fn from_hash_md5(hash_str: &str) -> Option<CICKind> {
        match hash_str {
            "900b4a5b68edb71f4c7ed52acd814fc5" => Some(CICKind::CIC_6101),
            "e24dd796b2fa16511521139d28c8356b" => Some(CICKind::CIC_6102_7101),
            "955894c2e40a698bf98a67b78a4e28fa" => Some(CICKind::CIC_7102),
            "319038097346e12c26c3c21b56f86f23" => Some(CICKind::CIC_X103),
            "ff22a296e55d34ab0a077dc2ba5f5796" => Some(CICKind::CIC_X105),
            "6460387749ac0bd925aa5430bc7864fe" => Some(CICKind::CIC_X106),
            _ => None,
        }
    }

    pub fn from_value(value: usize) -> Option<CICKind> {
        match value {
            6101 => Some(CICKind::CIC_6101),
            6102 | 7101 => Some(CICKind::CIC_6102_7101),
            7102 => Some(CICKind::CIC_7102),
            6103 | 7103 => Some(CICKind::CIC_X103),
            6105 | 7105 => Some(CICKind::CIC_X105),
            6106 | 7106 => Some(CICKind::CIC_X106),
            _ =>  None
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
        pub fn fromHashMd5(hash_str: &str) -> Option<super::CICKind> {
            super::CICKind::from_hash_md5(hash_str)
        }

        #[staticmethod]
        pub fn fromValue(value: usize) -> Option<super::CICKind> {
            super::CICKind::from_value(value)
        }

        #[getter]
        pub fn name(&self) -> &str {
            match self {
                super::CICKind::CIC_6101 => "CIC_6101",
                super::CICKind::CIC_6102_7101 => "CIC_6102_7101",
                super::CICKind::CIC_7102 => "CIC_7102",
                super::CICKind::CIC_X103 => "CIC_X103",
                super::CICKind::CIC_X105 => "CIC_X105",
                super::CICKind::CIC_X106 => "CIC_X106",
            }
        }
    }
}
