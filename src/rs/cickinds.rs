/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;

#[cfg_attr(feature = "python_bindings", pyclass(module = "ipl3checksum"))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CICKind {
    Cic6101,
    Cic6102_7101,
    Cic7102,
    CicX103, // Both 6103 and 7103
    // 6104/7104 does not exist
    CicX105, // Both 6105 and 7105
    CicX106, // Both 6106 and 7106
}

#[cfg_attr(feature = "python_bindings", pymethods)]
impl CICKind {
    pub fn get_seed(&self) -> u32 {
        match self {
            CICKind::Cic6101 => 0x3F,
            CICKind::Cic6102_7101 => 0x3F,
            CICKind::Cic7102 => 0x3F,
            CICKind::CicX103 => 0x78,
            CICKind::CicX105 => 0x91,
            CICKind::CicX106 => 0x85,
        }
    }

    pub fn get_magic(&self) -> u32 {
        match self {
            CICKind::Cic6101 => 0x5D588B65,
            CICKind::Cic6102_7101 => 0x5D588B65,
            CICKind::Cic7102 => 0x5D588B65,
            CICKind::CicX103 => 0x6C078965,
            CICKind::CicX105 => 0x5D588B65,
            CICKind::CicX106 => 0x6C078965,
        }
    }

    pub fn get_hash_md5(&self) -> &str {
        match self {
            CICKind::Cic6101 => "900b4a5b68edb71f4c7ed52acd814fc5",
            CICKind::Cic6102_7101 => "e24dd796b2fa16511521139d28c8356b",
            CICKind::Cic7102 => "955894c2e40a698bf98a67b78a4e28fa",
            CICKind::CicX103 => "319038097346e12c26c3c21b56f86f23",
            CICKind::CicX105 => "ff22a296e55d34ab0a077dc2ba5f5796",
            CICKind::CicX106 => "6460387749ac0bd925aa5430bc7864fe",
        }
    }

    #[cfg(feature = "python_bindings")]
    #[staticmethod]
    pub fn from_value(value: usize) -> Option<CICKind> {
        CICKind::from_value_impl(value)
    }

    #[cfg(not(feature = "python_bindings"))]
    pub fn from_value(value: usize) -> Option<CICKind> {
        CICKind::from_value_impl(value)
    }
}

impl CICKind {
    fn from_value_impl(value: usize) -> Option<CICKind> {
        match value {
            6101 => Some(CICKind::Cic6101),
            6102 | 7101 => Some(CICKind::Cic6102_7101),
            7102 => Some(CICKind::Cic7102),
            6103 | 7103 => Some(CICKind::CicX103),
            6105 | 7105 => Some(CICKind::CicX105),
            6106 | 7106 => Some(CICKind::CicX106),
            _ =>  None
        }
    }
}
