#ifndef IPL3CHECKSUM_CHECKSUM_H
#define IPL3CHECKSUM_CHECKSUM_H
#pragma once

#include <stddef.h>
#include <stdint.h>

#include "error.h"
#include "cickinds.h"

#ifdef __cplusplus
extern "C"
{
#endif

Ipl3Checksum_Error ipl3checksum_calculate_checksum(
    uint32_t *dst_checksum0,
    uint32_t *dst_checksum1,
    size_t rom_bytes_len,
    const uint8_t *rom_bytes,
    Ipl3Checksum_CICKind kind
);

Ipl3Checksum_Error ipl3checksum_calculate_checksum_autodetect(
    uint32_t *dst_checksum0,
    uint32_t *dst_checksum1,
    size_t rom_bytes_len,
    const uint8_t *rom_bytes
);

#ifdef __cplusplus
}
#endif

#endif
