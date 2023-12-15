/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

pub mod cickinds;

#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;

#[cfg(feature = "python_bindings")]
#[pymodule]
fn ipl3checksum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<cickinds::CICKind>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
}
