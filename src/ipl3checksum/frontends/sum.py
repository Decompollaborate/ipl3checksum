#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import argparse
from pathlib import Path

import ipl3checksum

def doChecksumize(romBytes: bytes) -> int:
    kind = ipl3checksum.detectCIC(romBytes)

    if kind is None:
        print(f"Unable to detect CIC kind")
        return 1

    print(f"Detected kind is '{kind.name}'")

    checksum = ipl3checksum.calculateChecksum(romBytes, kind)
    if checksum is None:
        print(f"Unable to calculate checksum")
        return 1

    chk0, chk1 = checksum
    print(f"Calculated checksum: {chk0:08X} {chk1:08X}")

    return 0


def processArguments(args: argparse.Namespace):
    romPath: Path = args.rom_path

    romBytes = romPath.read_bytes()

    exit(doChecksumize(romBytes))

def addSubparser(subparser: argparse._SubParsersAction[argparse.ArgumentParser]):
    parser = subparser.add_parser("sum", help="Calculates the ipl3 checksum of a big endian ROM.")

    parser.add_argument("rom_path", help="Path to a big endian ROM file", type=Path)

    parser.set_defaults(func=processArguments)
