#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023-2024 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

from .cickinds import CICKind

def calculateChecksum(romBytes: bytes, kind: CICKind) -> tuple[int, int]|None:
    """Calculates the checksum required by an official CIC of a N64 ROM.

    Args:
        romBytes (bytes): The bytes of the N64 ROM in big endian format. It must have a minimum size of 0x101000 bytes.
        kind (CICKind): The CIC kind variation used to calculate the checksum.

    Returns:
        tuple[int, int]|None: If no error happens then the calculated checksum is returned, stored as a tuple
        containing two 32-bits words. Otherwise, `None` is returned. Possible errors:
        - `romBytes` not being big enough
    """

def calculateChecksumAutodetect(romBytes: bytes) -> tuple[int, int]|None:
    """Calculates the checksum required by an official CIC of a N64 ROM.

    This function will try to autodetect the CIC kind automatically. If it fails to detect it then it will return `None`.

    Args:
        romBytes (bytes): The bytes of the N64 ROM in big endian format. It must have a minimum size of 0x101000 bytes.

    Returns:
        tuple[int, int]|None: If no error happens then the calculated checksum is returned, stored as a tuple
        containing two 32-bits words. Otherwise, `None` is returned. Possible errors:
        - `romBytes` not being big enough
        - Not able to detect the CIC kind
    """
