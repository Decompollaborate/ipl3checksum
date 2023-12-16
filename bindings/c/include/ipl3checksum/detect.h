#ifndef IPL3CHECKSUM_DETECT_H
#define IPL3CHECKSUM_DETECT_H
#pragma once

#include <stddef.h>
#include <stdint.h>

#include "error.h"
#include "cickinds.h"

#ifdef __cplusplus
extern "C"
{
#endif

Ipl3Checksum_Error ipl3checksum_detect_cic_raw(
    Ipl3Checksum_CICKind *dst_kind,
    size_t raw_bytes_len,
    const uint8_t *raw_bytes
);

Ipl3Checksum_Error ipl3checksum_detect_cic(
    Ipl3Checksum_CICKind *dst_kind,
    size_t rom_bytes_len,
    const uint8_t *rom_bytes
);

#ifdef __cplusplus
}
#endif

#endif
