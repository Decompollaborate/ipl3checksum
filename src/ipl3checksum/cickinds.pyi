#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023-2024 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

class CICKind():
    CIC_6101: CICKind
    CIC_6102_7101: CICKind
    CIC_7102: CICKind
    CIC_X103: CICKind # Both 6103 and 7103
    # 6104/7104 does not exist
    CIC_X105: CICKind # Both 6105 and 7105
    CIC_X106: CICKind # Both 6106 and 7106

    CIC_5101: CICKind # Aleck 64


    def getSeed(self) -> int:
        """
        Seed value set by the PIF ROM before the CPU (and the IPL3) is executed.

        https://n64brew.dev/wiki/PIF-NUS#IPL3_checksum_algorithm
        """

    def getMagic(self) -> int:
        """
        Magic value hardcoded inside the IPL3 itself
        """

    def getEntrypoint(self, header_entrypoint: int) -> int:
        """
        Calculates the actual entrypoint address based on the entrypoint specified on the header.

        CIC 7102 is a notable case since its IPL3 hardcodes it, ignoring the entrypoint from the header.
        """

    def getHashMd5(self) -> str:
        """
        Expected md5 hash of the IPL3 blob
        """

    @staticmethod
    def fromHashMd5(hash_str: str) -> CICKind|None:
        ...

    @property
    def name(self) -> str:
        ...

    @staticmethod
    def fromName(name: str) -> CICKind|None:
        ...

    @staticmethod
    def validNames() -> list[str]:
        ...

    @staticmethod
    def fromValue(value: int) -> CICKind|None:
        ...

    def calculateChecksum(self, romBytes: bytes) -> tuple[int, int]:
        """Calculates the checksum required by an official CIC of a N64 ROM.

        Args:
            romBytes (bytes): The bytes of the N64 ROM in big endian format. It must have a minimum size of 0x101000 bytes.

        Returns:
            tuple[int, int]: If no error happens then the calculated checksum is returned, stored as a tuple
            containing two 32-bits words. If an errors occurs an exception will be raised (see ipl3checksum.exceptions).
        """
