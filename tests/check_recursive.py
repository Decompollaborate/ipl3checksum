#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import argparse
import ipl3checksum
from pathlib import Path
import struct


print(f"Running ipl3checksum version {ipl3checksum.__version__}")

def checkChecksum(romPath: Path, romBytes: bytes) -> bool:
    print()
    print(romPath)

    binChecksum = struct.unpack_from(f">II", romBytes, 0x10)

    print(f"    Expected checksum is: 0x{binChecksum[0]:08X} 0x{binChecksum[1]:08X}")

    print("    Detecting CIC...")
    cicKind = ipl3checksum.detectCIC(romBytes)
    if cicKind is None:
        print(f"    Not able to detect CIC for {romPath}")
        return False

    print("    Calculating checksum...")
    calculatedChecksum = ipl3checksum.calculateChecksum(romBytes, cicKind)
    if calculatedChecksum is None:
        print(f"    Not able to calculate checksum for {romPath}")
        return False

    print(f"    Calculated checksum is: 0x{calculatedChecksum[0]:08X} 0x{calculatedChecksum[1]:08X}")

    print("    Checking checksum...")
    if calculatedChecksum[0] != binChecksum[0] or calculatedChecksum[1] != binChecksum[1]:
        print(f"    Wrong checksum for {romPath}")
        return False

    print(f"    {romPath} OK")

    return True

def recursePaths(folder: Path) -> int:
    errors = 0

    for subpath in sorted(folder.iterdir()):
        if subpath.name.startswith("."):
            continue

        print(subpath)

        if subpath.is_dir():
            errors += recursePaths(subpath)
            continue

        if subpath.parts[-2] == "drmario64" and subpath.name == "baserom.cn.z64":
            # iQue has a wrong checksum for some reason
            print(f"Skipping {subpath}")
            continue

        romBytes = subpath.read_bytes()
        romMagic = struct.unpack_from(f">I", romBytes, 0x0)[0]

        print(f"  Rom magic: {romMagic:08X}")
        if romMagic != 0x80371240:
            # Not an N64 rom
            print(f"Skipping {subpath}")
            continue

        ok = checkChecksum(subpath, romBytes)
        if not ok:
            errors += 1

    return errors


parser = argparse.ArgumentParser()
parser.add_argument("path")

args = parser.parse_args()

errors = recursePaths(Path(args.path))
print(f"Total errors: {errors}")
exit(errors)
