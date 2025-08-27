/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::error::Ipl3ChecksumError;

pub(crate) fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, Ipl3ChecksumError> {
    if offset % 4 != 0 {
        return Err(Ipl3ChecksumError::UnalignedRead { offset });
    }

    if offset + 4 > bytes.len() {
        return Err(Ipl3ChecksumError::OutOfBounds {
            offset,
            requested_bytes: 4,
            buffer_len: bytes.len(),
        });
    }

    match bytes[offset..offset + 4].try_into() {
        Ok(bytes) => Ok(u32::from_be_bytes(bytes)),
        Err(_error) => Err(Ipl3ChecksumError::ByteConversion { offset }),
    }
}

#[cfg(feature = "c_bindings")]
pub(crate) mod c_bindings {
    use crate::Ipl3ChecksumError;

    use alloc::vec::Vec;

    pub(crate) fn u8_vec_from_pointer_array(
        src_len: usize,
        src: *const u8,
    ) -> Result<Vec<u8>, Ipl3ChecksumError> {
        if src.is_null() {
            return Err(Ipl3ChecksumError::NullPointer);
        }

        let mut bytes = Vec::with_capacity(src_len);

        for i in 0..src_len {
            bytes.push(unsafe { *src.add(i) });
        }

        Ok(bytes)
    }

    pub(crate) fn static_str_from_c_string(
        c_str: *const core::ffi::c_char,
    ) -> Result<&'static str, Ipl3ChecksumError> {
        let converted = unsafe { core::ffi::CStr::from_ptr(c_str) };

        match converted.to_str() {
            Err(_) => Err(Ipl3ChecksumError::StringConversion),
            Ok(s) => Ok(s),
        }
    }

    pub(crate) fn free_c_string(s: *mut core::ffi::c_char) -> Result<(), Ipl3ChecksumError> {
        if s.is_null() {
            return Err(Ipl3ChecksumError::NullPointer);
        }

        unsafe {
            drop(alloc::ffi::CString::from_raw(s));
        }

        Ok(())
    }

    #[no_mangle]
    pub extern "C" fn ipl3checksum_free_string(s: *mut core::ffi::c_char) -> Ipl3ChecksumError {
        match free_c_string(s) {
            Err(e) => e,
            Ok(_) => Ipl3ChecksumError::Okay,
        }
    }

    pub(crate) fn c_string_from_rust_str(
        s: &str,
    ) -> Result<*mut core::ffi::c_char, Ipl3ChecksumError> {
        let c_str_song = match alloc::ffi::CString::new(s) {
            Err(_) => return Err(Ipl3ChecksumError::StringConversion),
            Ok(c_s) => c_s,
        };

        Ok(c_str_song.into_raw())
    }
}
