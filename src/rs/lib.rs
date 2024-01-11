/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

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

    #[pymodule]
    fn ipl3checksum(py: Python<'_>, m: &PyModule) -> PyResult<()> {
        // Classes
        m.add_class::<super::cickinds::CICKind>()?;

        // Free functions
        m.add_function(wrap_pyfunction!(
            super::checksum::python_bindings::calculateChecksum,
            m
        )?)?;
        m.add_function(wrap_pyfunction!(
            super::checksum::python_bindings::calculateChecksumAutodetect,
            m
        )?)?;
        m.add_function(wrap_pyfunction!(
            super::detect::python_bindings::detectCICRaw,
            m
        )?)?;
        m.add_function(wrap_pyfunction!(
            super::detect::python_bindings::detectCIC,
            m
        )?)?;

        // Exceptions

        register_exceptions_module(py, m)?;

        Ok(())
    }

    fn register_exceptions_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
        let child_module = PyModule::new(py, "exceptions")?;

        child_module.add(
            "Ipl3ChecksumError",
            py.get_type::<super::error::python_bindings::Ipl3ChecksumError>(),
        )?;

        child_module.add(
            "UnalignedRead",
            py.get_type::<super::error::python_bindings::UnalignedRead>(),
        )?;
        child_module.add(
            "ByteConversion",
            py.get_type::<super::error::python_bindings::ByteConversion>(),
        )?;
        child_module.add(
            "OutOfBounds",
            py.get_type::<super::error::python_bindings::OutOfBounds>(),
        )?;
        child_module.add(
            "BufferNotBigEnough",
            py.get_type::<super::error::python_bindings::BufferNotBigEnough>(),
        )?;
        child_module.add(
            "BufferSizeIsWrong",
            py.get_type::<super::error::python_bindings::BufferSizeIsWrong>(),
        )?;
        child_module.add(
            "UnableToDetectCIC",
            py.get_type::<super::error::python_bindings::UnableToDetectCIC>(),
        )?;

        parent_module.add_submodule(child_module)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
