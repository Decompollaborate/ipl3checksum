/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;

#[cfg(feature = "python_bindings")]
#[pyfunction]
fn testfunc(a: i32) -> String {
    a.to_string()
}

#[cfg(feature = "python_bindings")]
#[pymodule]
fn ipl3checksum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(testfunc, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
}
