#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2024 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations


class Ipl3ChecksumError(RuntimeError):
    """
    Base exception for all the exceptions raised by this library.
    """


class UnalignedRead(Ipl3ChecksumError):
    """
    An unaligned read happened.

    (This is probably a library bug, please report me).
    """

class ByteConversion(Ipl3ChecksumError):
    """
    Failed to convert bytes to words.

    (This is probably a library bug, please report me).
    """

class OutOfBounds(Ipl3ChecksumError):
    """
    Tried to access data out of bounds.

    (This is probably a library bug, please report me).
    """

class BufferNotBigEnough(Ipl3ChecksumError):
    """
    The input byte buffer is not big enough.

    The buffer can be larger than the expected size.

    The error runtime string specifies how big the buffer was expected to be.
    """

class BufferSizeIsWrong(Ipl3ChecksumError):
    """
    The input byte buffer didn't have the exact expected size.

    The error runtime string specifies the expected size.
    """

class UnableToDetectCIC(Ipl3ChecksumError):
    """
    Unable to detect CIC variant
    """
