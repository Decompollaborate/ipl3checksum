/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

pub mod checksum;
pub mod cickinds;
pub mod detect;

mod utils;

#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;

#[cfg(feature = "python_bindings")]
#[pymodule]
fn ipl3checksum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<cickinds::CICKind>()?;
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
    Ok(())
}

#[cfg(test)]
mod tests {}
