#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import ipl3checksum
from pathlib import Path
import struct

ipl3mapping = {
    "IPL3_6101":        ipl3checksum.IPL3Kind.IPL3_6101,
    "IPL3_6102_7101":   ipl3checksum.IPL3Kind.IPL3_6102_7101,
    "IPL3_7102":        ipl3checksum.IPL3Kind.IPL3_7102,
    "IPL3_X103":        ipl3checksum.IPL3Kind.IPL3_X103,
    "IPL3_X105":        ipl3checksum.IPL3Kind.IPL3_X105,
    "IPL3_X106":        ipl3checksum.IPL3Kind.IPL3_X106,
}

print(f"Running ipl3checksum version {ipl3checksum.__version__}")

for ipl3folder in sorted(Path("tests/dummytests").iterdir()):
    print(ipl3folder.name)

    kind = ipl3mapping[ipl3folder.name]

    for binPath in sorted(ipl3folder.iterdir()):
        print(binPath)

        print("    Reading...")
        binBytes = binPath.read_bytes()

        print("    Calculating checksum...")
        checksum = ipl3checksum.calculateChecksum(binBytes, kind)
        assert checksum is not None

        print(f"    Calculated checksum is: 0x{checksum[0]:08X} 0x{checksum[1]:08X}")

        print("    Checking checksum...")
        binChecksum = struct.unpack_from(f">II", binBytes, 0x10)

        print(f"    Expected checksum is: 0x{binChecksum[0]:08X} 0x{binChecksum[1]:08X}")

        assert checksum[0] == binChecksum[0]
        assert checksum[1] == binChecksum[1]

        print(f"    {binPath} OK")

        print()

    print()
