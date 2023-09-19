#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

__version_info__: tuple[int, int, int] = (0, 1, 0)
__version__ = ".".join(map(str, __version_info__)) + ".dev0"
__author__ = "Decompollaborate"

from . import utils as utils

from .checksum import calculateChecksum as calculateChecksum
from .cickinds import CICKind as CICKind
