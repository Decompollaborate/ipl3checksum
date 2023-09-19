#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import ipl3checksum
from pathlib import Path
import struct

print(f"Running ipl3checksum version {ipl3checksum.__version__}")

for binPath in sorted(Path("tests/dummytests").iterdir()):
    print(binPath)

    print("    Reading...")
    binBytes = binPath.read_bytes()

    print("    Calculating checksum...")
    # TODO: don't hardcode IPL3_6102_7101
    checksum = ipl3checksum.calculateChecksum(binBytes, ipl3checksum.IPL3Kind.IPL3_6102_7101)
    assert checksum is not None

    print(f"    Calculated checksum is: 0x{checksum[0]:08X} 0x{checksum[1]:08X}")

    print("    Checking checksum...")
    binChecksum = struct.unpack_from(f">II", binBytes, 0x10)

    print(f"    Expected checksum is: 0x{binChecksum[0]:08X} 0x{binChecksum[1]:08X}")

    assert checksum[0] == binChecksum[0]
    assert checksum[1] == binChecksum[1]

    print(f"    {binPath} OK")

    print()
