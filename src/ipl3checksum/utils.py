#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

import hashlib

def u32(value: int) -> int:
    return value & 0xFFFFFFFF

def getHashMd5(bytes: bytes) -> str:
    return str(hashlib.md5(bytes).hexdigest())
