#!/usr/bin/env python3

# SPDX-License-Identifier: CC0-1.0

from __future__ import annotations

import argparse
from pathlib import Path
import struct

parser = argparse.ArgumentParser()
parser.add_argument("rom")

args = parser.parse_args()
romPathArg = args.rom

romPath = Path(romPathArg)

romBytes = romPath.read_bytes()

def u32(value: int) -> int:
    value = value & 0xFFFFFFFF
    return value

def readWordFromRam(romWords: list[int], entrypointRam: int, ramAddr: int) -> int:
    offset = ramAddr - entrypointRam + 0x1000
    assert offset < 0x101000
    index = offset // 4
    assert index >= 0
    word = romWords[index]
    return word


def checksumfunc(romBytes: bytes, initial_s6 = 0x85):
    byteCount = len(romBytes)
    assert byteCount > 0x101000, f"0x{byteCount:X}"
    wordCount = byteCount // 4
    romWords = list(struct.unpack(f">{wordCount}I", romBytes))

    s6 = initial_s6

    a0 = romWords[8//4] - 0x200000
    entrypointRam = a0

    at = 0x6C078965
    lo = s6 * at

    ra = 0x100000

    v1 = 0
    t0 = 0

    t1 = a0

    t5 = 0x20

    v0 = u32(lo)

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

        v1 = u32(a3 + v0)

        at = u32(v1) < u32(a3)

        a1 = v1
        # if (at == 0) goto LA4000608;

        if at != 0:
            t2 = u32(t2 + 0x1)

        # LA4000608
        v1 = v0 & 0x1F
        t7 = u32(t5 - v1)


        t8 = u32(v0 >> t7)
        t6 = u32(v0 << v1)

        a0 = t6 | t8
        at = u32(a2) < u32(v0)
        a3 = a1

        t3 = t3 ^ v0

        s0 = u32(s0 + a0)
        # if (at == 0) goto LA400063C;
        if (at != 0):
            t9 = a3 ^ v0

            a2 = t9 ^ a2
            # goto LA4000640;

        # LA400063C:
        else:
            a2 = a2 ^ a0

        # LA4000640:
        t0 = u32(t0 + 0x4)
        t7 = v0 ^ s0
        t1 = u32(t1 + 0x4)


        t4 = u32(t7 + t4)
        # if (t0 != ra) goto LA40005F0;
        if t0 == ra:
            LA40005F0_loop = False


    t6 = u32(a3 * t2)
    a3 = u32(t6 + t3)
    t8 = u32(s0 * a2)
    s0 = u32(t8 + t4)

    return (a3, s0)


v1, v2 = checksumfunc(romBytes)
print(f"{v1:08X}")
print(f"{v2:08X}")
