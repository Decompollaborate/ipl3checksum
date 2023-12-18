#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import ipl3checksum
from pathlib import Path
import random
import struct


cics = [
    (ipl3checksum.CICKind.CIC_6101, ),
    (ipl3checksum.CICKind.CIC_6102_7101, ),
    (ipl3checksum.CICKind.CIC_7102, ),
    (ipl3checksum.CICKind.CIC_X103, ),
    (ipl3checksum.CICKind.CIC_X105, ),
    (ipl3checksum.CICKind.CIC_X106, ),
    (ipl3checksum.CICKind.CIC_5101, ),
]

for kind, in cics:
    print(f"Generating dummy for {kind}")

    random.seed(0xA1F)

    generatedBin = bytearray()

    for i in range(0x1000):
        generatedBin.append(0)

    for i in range(0x100000):
        generatedBin.append(random.randint(0, 0xFF))

    checksum = ipl3checksum.calculateChecksum(generatedBin, kind)
    assert checksum is not None
    w1, w2 = checksum

    struct.pack_into(f">II", generatedBin, 0x10, w1, w2)

    binPath = Path(f"tests/dummytests/{kind.name}")
    binPath.mkdir(parents=True, exist_ok=True)
    binPath /= "dummy.bin"
    binPath.write_bytes(generatedBin)
