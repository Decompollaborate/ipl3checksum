#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import argparse
import ipl3checksum
from pathlib import Path


parser = argparse.ArgumentParser()
parser.add_argument("rom", help="Path to a big endian rom")
parser.add_argument("-c", "--cic", help="The cic to use. If unrecognized or missing then the script will default to 6102")

args = parser.parse_args()
romPathArg = args.rom

romPath = Path(romPathArg)
cic = int(args.cic if args.cic is not None else 0)

romBytes = romPath.read_bytes()

ipl3kind = ipl3checksum.CICKind.fromValue(cic)
if ipl3kind is None:
    ipl3kind = ipl3checksum.CICKind.CIC_6102_7101

print(f"Using {ipl3kind.name}")

checksum = ipl3checksum.calculateChecksum(romBytes, ipl3kind)
assert checksum is not None

print(f"{checksum[0]:08X}")
print(f"{checksum[1]:08X}")
