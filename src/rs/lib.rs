/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

mod checksum;
mod cickinds;
mod detect;
mod error;
mod utils;
pub mod version;

pub use checksum::*;
pub use cickinds::*;
pub use detect::*;
pub use error::*;

#[cfg(feature = "python_bindings")]
mod python_bindings {
    use pyo3::prelude::*;

    use super::*;

    #[pymodule(gil_used = false)]
    fn ipl3checksum(m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Classes
        m.add_class::<cickinds::CICKind>()?;

        // Free functions
        m.add_function(wrap_pyfunction!(
            checksum::python_bindings::calculateChecksum,
            m
        )?)?;
        m.add_function(wrap_pyfunction!(
            checksum::python_bindings::calculateChecksumAutodetect,
            m
        )?)?;
        m.add_function(wrap_pyfunction!(detect::python_bindings::detectCICRaw, m)?)?;
        m.add_function(wrap_pyfunction!(detect::python_bindings::detectCIC, m)?)?;

        // Exceptions

        register_exceptions_module(m)?;

        Ok(())
    }

    fn register_exceptions_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
        let py = parent_module.py();
        let child_module = PyModule::new(py, "exceptions")?;

        child_module.add(
            "Ipl3ChecksumError",
            py.get_type::<error::python_bindings::Ipl3ChecksumError>(),
        )?;

        child_module.add(
            "UnalignedRead",
            py.get_type::<error::python_bindings::UnalignedRead>(),
        )?;
        child_module.add(
            "ByteConversion",
            py.get_type::<error::python_bindings::ByteConversion>(),
        )?;
        child_module.add(
            "OutOfBounds",
            py.get_type::<error::python_bindings::OutOfBounds>(),
        )?;
        child_module.add(
            "BufferNotBigEnough",
            py.get_type::<error::python_bindings::BufferNotBigEnough>(),
        )?;
        child_module.add(
            "BufferSizeIsWrong",
            py.get_type::<error::python_bindings::BufferSizeIsWrong>(),
        )?;
        child_module.add(
            "UnableToDetectCIC",
            py.get_type::<error::python_bindings::UnableToDetectCIC>(),
        )?;

        parent_module.add_submodule(&child_module)?;
        Ok(())
    }
}
