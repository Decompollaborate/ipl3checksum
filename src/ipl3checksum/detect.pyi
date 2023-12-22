#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

from .cickinds import CICKind

def detectCICRaw(rawBytes: bytes) -> CICKind|None:
    """Tries to detect an IPL3 binary.

    The argument to this function must be exactly the IPL3 binary.

    Args:
        rawBytes (bytes): IPL3 binary in big endian format.

    Returns:
        CICKind|None: The detected CIC kind, or `None` if was not able to detect the CIC kind.
    """


def detectCIC(romBytes: bytes) -> CICKind|None:
    """Tries to detect an IPL3 in a ROM.

    The argument to this function must be a ROM in big endian format.

    Args:
        romBytes (bytes): ROMbinary in big endian format.

    Returns:
        CICKind|None: The detected CIC kind, or `None` if was not able to detect the CIC kind.
    """
