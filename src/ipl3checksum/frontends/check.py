#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import argparse
from pathlib import Path
import struct

import ipl3checksum

def doCheck(romBytes: bytes, kindName: str | None) -> int:
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

    ogChk0, ogChk1 = struct.unpack_from(f">II", romBytes, 0x10)
    print(f"Checksum in ROM:     {ogChk0:08X} {ogChk1:08X}")

    checksum = ipl3checksum.calculateChecksum(romBytes, kind)
    if checksum is None:
        print(f"Unable to calculate checksum")
        return 1

    chk0, chk1 = checksum
    print(f"Calculated checksum: {chk0:08X} {chk1:08X}")

    if chk0 != ogChk0 or chk1 != ogChk1:
        print(f"Checksum doesn't match")
        return 1

    print("Checksum matches")
    return 0


def processArguments(args: argparse.Namespace):
    romPath: Path = args.rom_path
    kindName: str|None = args.kind

    romBytes = romPath.read_bytes()

    exit(doCheck(romBytes, kindName))

def addSubparser(subparser: argparse._SubParsersAction[argparse.ArgumentParser]):
    parser = subparser.add_parser("check", help="Checks if the checksum in the header matches the calculated checksum")

    parser.add_argument("rom_path", help="Path to a big endian ROM file", type=Path)

    parser.add_argument("-k", "-c", "--kind", "--cic", help="Used this variant to calculate the checksum instead of automatically detecting which kind the ROM uses", dest="kind", metavar="KIND", choices=ipl3checksum.CICKind.validNames())

    parser.set_defaults(func=processArguments)
