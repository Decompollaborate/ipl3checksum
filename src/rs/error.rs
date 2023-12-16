/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[cfg(feature = "python_bindings")]
use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;

/* This needs to be in sync with the C equivalent at `bindings/c/include/ipl3checksum/error.h` */
// repr is kinda complex and I may have got it wrong.
// I tried to follow the stuff at https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html
#[cfg_attr(feature = "c_bindings", repr(C))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, thiserror::Error)]
pub enum Ipl3ChecksumError {
    #[error("Not an error")]
    Okay,
    #[error("Unaligned read at offset 0x{offset:X}")]
    UnalignedRead { offset: usize },
    #[error("Failed to convert bytes at offset 0x{offset:X}")]
    ByteConversion { offset: usize },
    #[error("Tried to access data out of bounds at offset 0x{offset:X}. Requested bytes: 0x{requested_bytes:X}. Buffer length: 0x{buffer_len:X}")]
    OutOfBounds {
        offset: usize,
        requested_bytes: usize,
        buffer_len: usize,
    },
    #[error("Pointer is null")]
    NullPointer,
    #[error("The input byte buffer is not big enough. It should be at least 0x{expected_len:X} bytes long, but it was 0x{buffer_len:X} bytes")]
    BufferNotBigEnough {
        buffer_len: usize,
        expected_len: usize,
    },
    #[error("The input byte buffer didn't have the expected size. It should be exactly 0x{expected_len:X} bytes long, but it was 0x{buffer_len:X} bytes")]
    BufferSizeIsWrong {
        buffer_len: usize,
        expected_len: usize,
    },
    #[error("Unable to detect CIC variant")]
    UnableToDetectCIC,
    #[error("Failed to convert a FFI string")]
    StringConversion,
}

#[cfg(feature = "python_bindings")]
impl std::convert::From<Ipl3ChecksumError> for PyErr {
    fn from(err: Ipl3ChecksumError) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}
