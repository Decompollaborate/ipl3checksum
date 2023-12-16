#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
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


    def getSeed(self) -> int:
        """
        Seed value set by the PIF ROM before the CPU (and the IPL3) is executed.
        https://n64brew.dev/wiki/PIF-NUS#IPL3_checksum_algorithm
        """

    def getMagic(self) -> int:
        """
        Magic value hardcoded inside the IPL3 itself
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
    def fromValue(value: int) -> CICKind|None:
        ...
