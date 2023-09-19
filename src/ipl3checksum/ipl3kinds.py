#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import enum

class IPL3Kind(enum.Enum):
    IPL3_6101 = enum.auto()
    IPL3_6102_7101 = enum.auto()
    IPL3_7102 = enum.auto()
    IPL3_X103 = enum.auto() # Both 6103 and 7103
    # 6104/7104 does not exist
    IPL3_X105 = enum.auto() # Both 6105 and 7105
    IPL3_X106 = enum.auto() # Both 6106 and 7106


    def getSeed(self) -> int:
        """
        Seed value set by the PIF ROM before the CPU (and the IPL3) is executed.
        https://n64brew.dev/wiki/PIF-NUS#IPL3_checksum_algorithm
        """
        return IPL3Seeds[self]

    def getMagic(self) -> int:
        """
        Magic value hardcoded inside the IPL3 itself
        """
        return IPL3Magics[self]

    def getHashMd5(self) -> str:
        """
        Expected md5 hash of the IPL3 blob
        """
        return IPL3HashMd5[self]

IPL3Seeds: dict[IPL3Kind, int] = {
    IPL3Kind.IPL3_6101:      0x3F,
    IPL3Kind.IPL3_6102_7101: 0x3F,
    IPL3Kind.IPL3_7102:      0x3F,
    IPL3Kind.IPL3_X103:      0x78,
    IPL3Kind.IPL3_X105:      0x91,
    IPL3Kind.IPL3_X106:      0x85,
}

IPL3Magics: dict[IPL3Kind, int] = {
    IPL3Kind.IPL3_6101:      0x5D588B65,
    IPL3Kind.IPL3_6102_7101: 0x5D588B65,
    IPL3Kind.IPL3_7102:      0x5D588B65,
    IPL3Kind.IPL3_X103:      0x6C078965,
    IPL3Kind.IPL3_X105:      0x5D588B65,
    IPL3Kind.IPL3_X106:      0x6C078965,
}

IPL3HashMd5: dict[IPL3Kind, str] = {
    IPL3Kind.IPL3_6101:      "900b4a5b68edb71f4c7ed52acd814fc5",
    IPL3Kind.IPL3_6102_7101: "e24dd796b2fa16511521139d28c8356b",
    IPL3Kind.IPL3_7102:      "955894c2e40a698bf98a67b78a4e28fa",
    IPL3Kind.IPL3_X103:      "319038097346e12c26c3c21b56f86f23",
    IPL3Kind.IPL3_X105:      "ff22a296e55d34ab0a077dc2ba5f5796",
    IPL3Kind.IPL3_X106:      "6460387749ac0bd925aa5430bc7864fe",
}
