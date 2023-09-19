#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import struct

from . import utils
from .cickinds import CICKind


def readWordFromRam(romWords: list[int], entrypointRam: int, ramAddr: int) -> int:
    return romWords[utils.u32(ramAddr - entrypointRam + 0x1000) // 4]


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

    if len(romBytes) < 0x101000:
        return None

    romWords = list(struct.unpack_from(f">{0x101000//4}I", romBytes))

    seed = kind.getSeed()
    magic = kind.getMagic()

    s6 = seed

    a0 = romWords[8//4]
    if kind == CICKind.CIC_X103:
        a0 -= 0x100000
    if kind == CICKind.CIC_X106:
        a0 -= 0x200000
    entrypointRam = a0

    at = magic
    lo = s6 * at

    if kind == CICKind.CIC_X105:
        s6 = 0xA0000200

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
        if kind == CICKind.CIC_X105:
            # ipl3 6105 copies 0x330 bytes from the ROM's offset 0x000554 (or offset 0x000514 into IPL3) to vram 0xA0000004
            t7 = romWords[(s6 - 0xA0000004 + 0x000554) // 4]

            t0 = utils.u32(t0 + 0x4)
            s6 = utils.u32(s6 + 0x4)
            t7 = v0 ^ t7

            t4 = utils.u32(t7 + t4)

            t7 = 0xA00002FF

            t1 = utils.u32(t1 + 0x4)

            s6 = utils.u32(s6 & t7)
        else:
            t0 = utils.u32(t0 + 0x4)
            t7 = v0 ^ s0
            t1 = utils.u32(t1 + 0x4)

            t4 = utils.u32(t7 + t4)

        # if (t0 != ra) goto LA40005F0;
        if t0 == ra:
            LA40005F0_loop = False


    if kind == CICKind.CIC_X103:
        t6 = a3 ^ t2
        a3 = utils.u32(t6 + t3)
        t8 = s0 ^ a2
        s0 = utils.u32(t8 + t4)
    elif kind == CICKind.CIC_X106:
        t6 = utils.u32(a3 * t2)
        a3 = utils.u32(t6 + t3)
        t8 = utils.u32(s0 * a2)
        s0 = utils.u32(t8 + t4)
    else:
        t6 = a3 ^ t2
        a3 = t6 ^ t3
        t8 = s0 ^ a2
        s0 = t8 ^ t4

    return (a3, s0)
