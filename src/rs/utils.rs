/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
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

pub(crate) fn read_u32_vec(
    bytes: &[u8],
    offset: usize,
    len: usize,
) -> Result<Vec<u32>, Ipl3ChecksumError> {
    let mut ret = Vec::with_capacity(len);

    for i in 0..len {
        ret.push(read_u32(bytes, offset + i * 4).unwrap());
    }

    Ok(ret)
}

pub(crate) fn get_hash_md5(bytes: &[u8]) -> String {
    format!("{:x}", md5::compute(bytes))
}
