/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::cickinds::CICKind;
use crate::utils;

pub fn detectCICRaw(rawBytes: &[u8]) -> Option<CICKind> {
    /*Tries to detect an IPL3 binary.

    The argument to this function must be exactly the IPL3 binary, stripping the rest of the ROM.

    Args:
        rawBytes (bytes): IPL3 binary in big endian format.

    Returns:
        CICKind|None: The detected CIC kind, or `None` if was not able to detect the CIC kind.
    */

    if rawBytes.len() != 0xFC0 {
        return None;
    }

    let bytesHash = utils::get_hash_md5(rawBytes);

    CICKind::from_hash_md5(&bytesHash)
}

pub fn detectCIC(romBytes: &[u8]) -> Option<CICKind> {
    /*Tries to detect an IPL3 in a ROM.

    The argument to this function must be a ROM in big endian format.

    Args:
        romBytes (bytes): ROMbinary in big endian format.

    Returns:
        CICKind|None: The detected CIC kind, or `None` if was not able to detect the CIC kind.
    */

    detectCICRaw(&romBytes[0x40..0x1000])
}
