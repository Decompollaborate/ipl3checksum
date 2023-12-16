/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef UTILS_H
#define UTILS_H
#pragma once

#include <stddef.h>
#include <stdint.h>

#include "ipl3checksum.h"

uint8_t *read_binary_file(const char *path, size_t *size);

const char *get_ipl3checksum_error_str(Ipl3Checksum_Error error);

#endif
