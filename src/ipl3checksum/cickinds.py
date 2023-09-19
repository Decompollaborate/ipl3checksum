#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import enum

class CICKind(enum.Enum):
    CIC_6101 = enum.auto()
    CIC_6102_7101 = enum.auto()
    CIC_7102 = enum.auto()
    CIC_X103 = enum.auto() # Both 6103 and 7103
    # 6104/7104 does not exist
    CIC_X105 = enum.auto() # Both 6105 and 7105
    CIC_X106 = enum.auto() # Both 6106 and 7106


    def getSeed(self) -> int:
        """
        Seed value set by the PIF ROM before the CPU (and the IPL3) is executed.
        https://n64brew.dev/wiki/PIF-NUS#IPL3_checksum_algorithm
        """
        return CICSeeds[self]

    def getMagic(self) -> int:
        """
        Magic value hardcoded inside the IPL3 itself
        """
        return CICMagics[self]

    def getHashMd5(self) -> str:
        """
        Expected md5 hash of the IPL3 blob
        """
        return CICHashMd5[self]

    @staticmethod
    def fromValue(value: int) -> CICKind|None:
        if value == 6102 or value == 7101:
            return CICKind.CIC_6102_7101
        if value == 6101:
            return CICKind.CIC_6101
        if value == 7102:
            return CICKind.CIC_7102
        if value == 6103 or value == 7103:
            return CICKind.CIC_X103
        if value == 6105 or value == 7105:
            return CICKind.CIC_X105
        if value == 6106 or value == 7106:
            return CICKind.CIC_X106

        return None


CICSeeds: dict[CICKind, int] = {
    CICKind.CIC_6101:      0x3F,
    CICKind.CIC_6102_7101: 0x3F,
    CICKind.CIC_7102:      0x3F,
    CICKind.CIC_X103:      0x78,
    CICKind.CIC_X105:      0x91,
    CICKind.CIC_X106:      0x85,
}

CICMagics: dict[CICKind, int] = {
    CICKind.CIC_6101:      0x5D588B65,
    CICKind.CIC_6102_7101: 0x5D588B65,
    CICKind.CIC_7102:      0x5D588B65,
    CICKind.CIC_X103:      0x6C078965,
    CICKind.CIC_X105:      0x5D588B65,
    CICKind.CIC_X106:      0x6C078965,
}

CICHashMd5: dict[CICKind, str] = {
    CICKind.CIC_6101:      "900b4a5b68edb71f4c7ed52acd814fc5",
    CICKind.CIC_6102_7101: "e24dd796b2fa16511521139d28c8356b",
    CICKind.CIC_7102:      "955894c2e40a698bf98a67b78a4e28fa",
    CICKind.CIC_X103:      "319038097346e12c26c3c21b56f86f23",
    CICKind.CIC_X105:      "ff22a296e55d34ab0a077dc2ba5f5796",
    CICKind.CIC_X106:      "6460387749ac0bd925aa5430bc7864fe",
}
