#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import struct

from . import utils
from .ipl3kinds import IPL3Kind


def readWordFromRam(romWords: list[int], entrypointRam: int, ramAddr: int) -> int:
    return romWords[(ramAddr - entrypointRam + 0x1000) // 4]


def calculateChecksum(romBytes: bytes, kind: IPL3Kind) -> tuple[int, int]|None:
    """Calculates the checksum required by an official IPL3 loader of a N64 ROM.

    Args:
        romBytes (bytes): The bytes of the N64 ROM in big endian format. It must have a minimum size of 0x101000 bytes.
        kind (IPL3Kind): The IPL3 kind variation used to calculate the checksum.

    Returns:
        tuple[int, int]|None: If no error happens then the calculated checksum is returned, stored as a tuple
        containing two 32-bits words. Otherwise, `None` is returned. Possible errors:
        - `romBytes` not being big enough
    """

    assert kind != IPL3Kind.IPL3_6103
    assert kind != IPL3Kind.IPL3_7103

    assert kind != IPL3Kind.IPL3_6105
    assert kind != IPL3Kind.IPL3_7105

    assert kind != IPL3Kind.IPL3_6106
    assert kind != IPL3Kind.IPL3_7106

    if len(romBytes) < 0x101000:
        return None

    romWords = list(struct.unpack(f">{0x101000//4}I", romBytes))

    seed = kind.getSeed()
    magic = kind.getMagic()

    s6 = seed

    a0 = romWords[8//4]
    entrypointRam = a0

    at = magic
    lo = s6 * at

    ra = 0x100000

    v1 = 0
    t0 = 0

    t1 = a0

    t5 = 0x20

    v0 = utils.u32(lo)

    v0 += 1

    a3 = v0
    t2 = v0
    t3 = v0
    s0 = v0
    a2 = v0
    t4 = v0

    # poor man's do while
    LA40005F0_loop = True
    while LA40005F0_loop:
        # v0 = *t1
        v0 = readWordFromRam(romWords, entrypointRam, t1)

        v1 = utils.u32(a3 + v0)

        at = utils.u32(v1) < utils.u32(a3)

        a1 = v1
        # if (at == 0) goto LA4000608;

        if at != 0:
            t2 = utils.u32(t2 + 0x1)

        # LA4000608
        v1 = v0 & 0x1F
        t7 = utils.u32(t5 - v1)


        t8 = utils.u32(v0 >> t7)
        t6 = utils.u32(v0 << v1)

        a0 = t6 | t8
        at = utils.u32(a2) < utils.u32(v0)
        a3 = a1

        t3 = t3 ^ v0

        s0 = utils.u32(s0 + a0)
        # if (at == 0) goto LA400063C;
        if (at != 0):
            t9 = a3 ^ v0

            a2 = t9 ^ a2
            # goto LA4000640;

        # LA400063C:
        else:
            a2 = a2 ^ a0

        # LA4000640:
        t0 = utils.u32(t0 + 0x4)
        t7 = v0 ^ s0
        t1 = utils.u32(t1 + 0x4)


        t4 = utils.u32(t7 + t4)
        # if (t0 != ra) goto LA40005F0;
        if t0 == ra:
            LA40005F0_loop = False


    t6 = a3 ^ t2
    a3 = t6 ^ t3
    t8 = s0 ^ a2
    s0 = t8 ^ t4

    return (a3, s0)
