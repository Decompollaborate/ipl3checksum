#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

# Version should be synced with pyproject.toml and Cargo.toml
__version_info__: tuple[int, int, int] = (1, 1, 0)
__version__ = ".".join(map(str, __version_info__)) + ".dev0"
__author__ = "Decompollaborate"

from .ipl3checksum import *
