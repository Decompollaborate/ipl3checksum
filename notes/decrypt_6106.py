#!/usr/bin/env python3

# SPDX-License-Identifier: CC0-1.0

D_A40004F0 = [
    # Fill with 0x2BC bytes at rom offset 0x0004F0 from cic 6106
]

D_A0000000 = [0 for x in D_A40004F0]

def u32(value: int) -> int:
    value = value & 0xFFFFFFFF
    return value

initial_s6 = 0x85


SEED = 0x0260BCD5

s6 = initial_s6

t4 = u32(u32(s6 * SEED) + 1)

for i in range(len(D_A40004F0)):
    t5 = D_A40004F0[i]
    D_A0000000[i] = t5 ^ t4

    t4 = u32(t4 * SEED)

wordCount = len(D_A0000000)

from pathlib import Path
import struct
newBytes = struct.pack(f">{wordCount}I", *D_A0000000)

Path("test.6106.bin").write_bytes(newBytes)
