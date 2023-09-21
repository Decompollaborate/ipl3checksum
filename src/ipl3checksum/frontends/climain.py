#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import argparse

from .. import __version__


def ipl3checksumMain():
    parser = argparse.ArgumentParser(description="Interface to call any of the ipl3checksum's CLI utilities", prog="ipl3checksum")

    parser.add_argument("-V", "--version", action="version", version=f"%(prog)s {__version__}")

    # subparsers = parser.add_subparsers(description="action", help="The CLI utility to run", required=True)

    # ipl3checksum.frontends.utility.addSubparser(subparsers)

    args = parser.parse_args()
    # args.func(args)


if __name__ == "__main__":
    ipl3checksumMain()
