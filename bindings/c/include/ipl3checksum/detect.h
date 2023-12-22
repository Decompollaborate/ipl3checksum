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

/**
 * Tries to detect an IPL3 binary.
 *
 * The argument to this function must be exactly the IPL3 binary, meaning the
 * binary size must match exactly the one of an IPL3 binary.
 *
 * ## Arguments
 *
 * * `dst_kind` - Pointer where the detected kind will be set to.
 * * `raw_bytes_len` - Bytes length of the input `raw_bytes`.
 * * `raw_bytes` - Bytes of an IPL3 binary in big endian format.
 *
 * ## Return
 *
 * * `Ipl3Checksum_Error` indicating either a successful execution or the cause
 *   for failing. If execution fails then `dst_kind` is left untouched.
 */
Ipl3Checksum_Error ipl3checksum_detect_cic_raw(
    Ipl3Checksum_CICKind *dst_kind,
    size_t raw_bytes_len,
    const uint8_t *raw_bytes
);

/**
 * Tries to detect an IPL3 in a ROM.
 *
 * The argument to this function must be a ROM in big endian format.
 *
 * ## Arguments
 *
 * * `dst_kind` - Pointer where the detected kind will be set to.
 * * `raw_bytes_len` - Bytes length of the input `rom_bytes`.
 * * `rom_bytes` - ROM binary in big endian format.
 *
 * ## Return
 *
 * * `Ipl3Checksum_Error` indicating either a successful execution or the cause
 *   for failing. If execution fails then `dst_kind` is left untouched.
 */
Ipl3Checksum_Error ipl3checksum_detect_cic(
    Ipl3Checksum_CICKind *dst_kind,
    size_t rom_bytes_len,
    const uint8_t *rom_bytes
);

#ifdef __cplusplus
}
#endif

#endif
