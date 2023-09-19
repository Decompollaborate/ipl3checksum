#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import enum

class IPL3Kind(enum.Enum):
    IPL3_6101 = enum.auto()

    IPL3_6102 = enum.auto()
    IPL3_7101 = enum.auto()

    IPL3_7102 = enum.auto()

    IPL3_6103 = enum.auto()
    IPL3_7103 = enum.auto()

    # 6104/7104 does not exist

    IPL3_6105 = enum.auto()
    IPL3_7105 = enum.auto()

    IPL3_6106 = enum.auto()
    IPL3_7106 = enum.auto()


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

IPL3Seeds: dict[IPL3Kind, int] = {
    IPL3Kind.IPL3_6101: 0x3F,

    IPL3Kind.IPL3_6102: 0x3F,
    IPL3Kind.IPL3_7101: 0x3F,

    IPL3Kind.IPL3_7102: 0x3F,

    IPL3Kind.IPL3_6103: 0x78,
    IPL3Kind.IPL3_7103: 0x78,

    IPL3Kind.IPL3_6105: 0x91,
    IPL3Kind.IPL3_7105: 0x91,

    IPL3Kind.IPL3_6106: 0x85,
    IPL3Kind.IPL3_7106: 0x85,
}

IPL3Magics: dict[IPL3Kind, int] = {
    IPL3Kind.IPL3_6101: 0x5D588B65,

    IPL3Kind.IPL3_6102: 0x5D588B65,
    IPL3Kind.IPL3_7101: 0x5D588B65,

    IPL3Kind.IPL3_7102: 0x5D588B65,

    IPL3Kind.IPL3_6103: 0x6C078965,
    IPL3Kind.IPL3_7103: 0x6C078965,

    IPL3Kind.IPL3_6105: 0x5D588B65,
    IPL3Kind.IPL3_7105: 0x5D588B65,

    IPL3Kind.IPL3_6106: 0x6C078965,
    IPL3Kind.IPL3_7106: 0x6C078965,
}
