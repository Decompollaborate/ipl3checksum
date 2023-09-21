#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

__version_info__: tuple[int, int, int] = (1, 0, 1)
__version__ = ".".join(map(str, __version_info__))
__author__ = "Decompollaborate"

from . import utils as utils

from .cickinds import CICKind as CICKind

from .checksum import calculateChecksum as calculateChecksum
from .checksum import calculateChecksumAutodetect as calculateChecksumAutodetect

from .detect import detectCIC as detectCIC
from .detect import detectCICRaw as detectCICRaw
