/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* This needs to be in sync with the C equivalent at `bindings/c/include/ipl3checksum/error.h` */
// repr is kinda complex and I may have got it wrong.
// I tried to follow the stuff at https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html
#[cfg_attr(feature = "c_bindings", repr(C))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, thiserror::Error)]
pub enum Ipl3ChecksumError {
    #[cfg(feature = "c_bindings")]
    #[error("Not an error")]
    Okay,
    #[cfg(feature = "c_bindings")]
    #[error("Pointer is null")]
    NullPointer,
    #[cfg(feature = "c_bindings")]
    #[error("Failed to convert a FFI string")]
    StringConversion,

    #[error("Unaligned read at offset 0x{offset:X}. \n (This is probably a library bug, please report me)")]
    UnalignedRead { offset: usize },
    #[error("Failed to convert bytes at offset 0x{offset:X} \n (This is probably a library bug, please report me)")]
    ByteConversion { offset: usize },
    #[error("Tried to access data out of bounds at offset 0x{offset:X}. Requested bytes: 0x{requested_bytes:X}. Buffer length: 0x{buffer_len:X} \n (This is probably a library bug, please report me)")]
    OutOfBounds {
        offset: usize,
        requested_bytes: usize,
        buffer_len: usize,
    },
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
}

#[cfg(feature = "python_bindings")]
pub(crate) mod python_bindings {
    use pyo3::exceptions::PyRuntimeError;
    use pyo3::prelude::*;

    pyo3::create_exception!(ipl3checksum, Ipl3ChecksumError, PyRuntimeError);

    pyo3::create_exception!(ipl3checksum, UnalignedRead, Ipl3ChecksumError);
    pyo3::create_exception!(ipl3checksum, ByteConversion, Ipl3ChecksumError);
    pyo3::create_exception!(ipl3checksum, OutOfBounds, Ipl3ChecksumError);
    pyo3::create_exception!(ipl3checksum, BufferNotBigEnough, Ipl3ChecksumError);
    pyo3::create_exception!(ipl3checksum, BufferSizeIsWrong, Ipl3ChecksumError);
    pyo3::create_exception!(ipl3checksum, UnableToDetectCIC, Ipl3ChecksumError);

    impl std::convert::From<super::Ipl3ChecksumError> for PyErr {
        fn from(err: super::Ipl3ChecksumError) -> PyErr {
            match err {
                super::Ipl3ChecksumError::UnalignedRead { .. } => {
                    UnalignedRead::new_err(err.to_string())
                }
                super::Ipl3ChecksumError::ByteConversion { .. } => {
                    ByteConversion::new_err(err.to_string())
                }
                super::Ipl3ChecksumError::OutOfBounds { .. } => {
                    OutOfBounds::new_err(err.to_string())
                }
                super::Ipl3ChecksumError::BufferNotBigEnough { .. } => {
                    BufferNotBigEnough::new_err(err.to_string())
                }
                super::Ipl3ChecksumError::BufferSizeIsWrong { .. } => {
                    BufferSizeIsWrong::new_err(err.to_string())
                }
                super::Ipl3ChecksumError::UnableToDetectCIC => {
                    UnableToDetectCIC::new_err(err.to_string())
                }
                #[cfg(feature = "c_bindings")]
                super::Ipl3ChecksumError::Okay
                | super::Ipl3ChecksumError::NullPointer
                | super::Ipl3ChecksumError::StringConversion => {
                    Ipl3ChecksumError::new_err(err.to_string())
                }
            }
        }
    }
}
