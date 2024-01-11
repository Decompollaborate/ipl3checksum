#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2024 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import ipl3checksum

# We want this buffer to be small so it triggers an exception
b = bytes([0,0,0,0])

try:
    checksum = ipl3checksum.CICKind.CIC_6102_7101.calculateChecksum(b)

    print(f"This code shouldn't run")
    print(f"{checksum[0]:08X} {checksum[1]:08X}")
    raise RuntimeError()

except ipl3checksum.exceptions.BufferNotBigEnough as e:
    print("We triggered and succesfully catched an exception!")
    print(f"  e: {e}")
