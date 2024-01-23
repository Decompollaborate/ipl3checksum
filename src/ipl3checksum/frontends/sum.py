#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023-2024 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import argparse
from pathlib import Path
import struct

import ipl3checksum

def doSum(romBytes: bytes, kindName: str | None, update: bool, outputPath: Path) -> int:
    if kindName is None:
        # Detect kind if none was specified by the user
        kind = ipl3checksum.detectCIC(romBytes)
        if kind is None:
            print(f"Unable to detect CIC kind")
            return 1
        print(f"Detected kind is '{kind.name}'")
    else:
        kind = ipl3checksum.CICKind.fromName(kindName)
        if kind is None:
            print(f"Invalid choice for cic kind. Valid choices: {ipl3checksum.CICKind.validNames()}")
            return 1

    checksum = ipl3checksum.calculateChecksum(romBytes, kind)
    if checksum is None:
        print(f"Unable to calculate checksum")
        return 1

    chk0, chk1 = checksum
    print(f"Calculated checksum: {chk0:08X} {chk1:08X}")

    if update:
        print(f"Writing updated ROM to '{outputPath}'")
        outputPath.parent.mkdir(parents=True, exist_ok=True)
        with outputPath.open("wb") as f:
            f.write(romBytes[:0x10])
            f.write(struct.pack(f">II", chk0, chk1))
            f.write(romBytes[0x18:])

    return 0


def processArguments(args: argparse.Namespace):
    romPath: Path = args.rom_path
    kindName: str|None = args.kind
    update: bool = args.update
    outputPath: Path|None = args.output

    romBytes = romPath.read_bytes()

    if outputPath is None:
        outputPath = romPath

    exit(doSum(romBytes, kindName, update, outputPath))

def addSubparser(subparser: argparse._SubParsersAction[argparse.ArgumentParser]):
    parser = subparser.add_parser("sum", help="Calculates the ipl3 checksum of a big endian ROM by detecting the CIC it uses and optionally update it.")

    parser.add_argument("rom_path", help="Path to a big endian ROM file", type=Path)

    parser.add_argument("-k", "-c", "--kind", "--cic", help="Use this variant to calculate the checksum instead of automatically detecting which kind the ROM uses", dest="kind", metavar="KIND", choices=ipl3checksum.CICKind.validNames())
    parser.add_argument("-u", "--update", help="Updates the ROM with the calculated checksum. This option modifies the input rom unless `--output` is used", action="store_true")
    parser.add_argument("-o", "--output", help="Path to write the updated ROM. This option is ignored if `--update` is not used", type=Path)

    parser.set_defaults(func=processArguments)
