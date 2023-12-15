/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use md5;

pub(crate) fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    if offset % 4 != 0 {
        panic!("Unaligned read");
    }

    if offset + 4 > bytes.len() {
        panic!("Out of bounds. Offset {:X}, len {:X}", offset, bytes.len());
    }

    /*
    match bytes[offset..offset + 4].try_into() {
        Ok(bytes) => u32::from_be_bytes(bytes),
        Err(_error) => todo!(),
    }
    */

    u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

pub(crate) fn read_u32_vec(bytes: &[u8], offset: usize, len: usize) -> Vec<u32> {
    let mut ret = vec![0;len];

    for i in 0..len {
        ret[i] = read_u32(bytes, offset + i*4);
    }

    ret
}

pub(crate) fn get_hash_md5(bytes: &[u8]) -> String {
    format!("{:x}", md5::compute(bytes))
}
