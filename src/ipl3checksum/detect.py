#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

from . import utils
from .cickinds import CICKind, CICHashMd5


def detectCICRaw(rawBytes: bytes) -> CICKind|None:
    """Tries to detect an IPL3 binary.

    The argument to this function must be exactly the IPL3 binary, stripping the rest of the ROM.

    Args:
        rawBytes (bytes): IPL3 binary in big endian format.

    Returns:
        CICKind|None: The detected CIC kind, or `None` if was not able to detect the CIC kind.
    """

    if len(rawBytes) < 0xFC0:
        return None

    bytesHash = utils.getHashMd5(rawBytes[:0xFC0])

    for kind, expectedHash in CICHashMd5.items():
        if bytesHash == expectedHash:
            return kind

    return None


def detectCIC(romBytes: bytes) -> CICKind|None:
    """Tries to detect an IPL3 in a ROM.

    The argument to this function must be a ROM in big endian format.

    Args:
        romBytes (bytes): ROMbinary in big endian format.

    Returns:
        CICKind|None: The detected CIC kind, or `None` if was not able to detect the CIC kind.
    """

    return detectCICRaw(romBytes[0x40:0x1000])
