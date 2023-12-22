#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import argparse
from pathlib import Path

import ipl3checksum

def doDetectCic(romBytes: bytes) -> int:
    kind = ipl3checksum.detectCIC(romBytes)

    if kind is None:
        print(f"Unable to detect CIC kind")
        return 1

    print(f"Detected kind is '{kind.name}'")

    return 0


def processArguments(args: argparse.Namespace):
    romPath: Path = args.rom_path

    romBytes = romPath.read_bytes()

    exit(doDetectCic(romBytes))

def addSubparser(subparser: argparse._SubParsersAction[argparse.ArgumentParser]):
    parser = subparser.add_parser("detect_cic", help="Detects the CIC variant of a given rom")

    parser.add_argument("rom_path", help="Path to a big endian ROM file", type=Path)

    parser.set_defaults(func=processArguments)
