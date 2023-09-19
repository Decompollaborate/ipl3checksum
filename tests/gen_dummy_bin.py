#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import ipl3checksum
from pathlib import Path
import random
import struct


# TODO: don't hardcode 6102

random.seed(0xA1F)

generatedBin = bytearray()

for i in range(0x1000):
    generatedBin.append(0)

for i in range(0x100000):
    generatedBin.append(random.randint(0, 0xFF))

checksum = ipl3checksum.calculateChecksum(generatedBin, ipl3checksum.IPL3Kind.IPL3_6102_7101)
assert checksum is not None
w1, w2 = checksum

struct.pack_into(f">II", generatedBin, 0x10, w1, w2)

Path(f"tests/dummytests/dummy.6102.bin").write_bytes(generatedBin)
