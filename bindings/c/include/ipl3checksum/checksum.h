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

/**
 * Calculates the checksum required by an official CIC of a N64 ROM.
 *
 * ## Arguments
 *
 * * `dst_checksum0` - Pointer where the first word of the calculated checksum will be placed.
 * * `dst_checksum1` - Pointer where the second word of the calculated checksum will be placed.
 * * `rom_bytes_len` - Bytes length of the input `rom_bytes`.
 * * `rom_bytes` - The bytes of the N64 ROM in big endian format. It must have a minimum size of 0x101000 bytes.
 * * `kind` - The CIC kind variation used to calculate the checksum.
 *
 * ## Return
 *
 * * `Ipl3Checksum_Error` indicating either a successful execution or the cause for failing.
 *   If execution fails then `dst_checksum0` and `dst_checksum1` are left untouched.
 */
Ipl3Checksum_Error ipl3checksum_calculate_checksum(
    uint32_t *dst_checksum0,
    uint32_t *dst_checksum1,
    size_t rom_bytes_len,
    const uint8_t *rom_bytes,
    Ipl3Checksum_CICKind kind
);

/**
 * Calculates the checksum required by an official CIC of a N64 ROM.
 *
 * This function will try to autodetect the CIC kind automatically.
 * If it fails to detect it then an error will be returned.
 *
 * ## Arguments
 *
 * * `dst_checksum0` - Pointer where the first word of the calculated checksum will be placed.
 * * `dst_checksum1` - Pointer where the second word of the calculated checksum will be placed.
 * * `rom_bytes_len` - Bytes length of the input `rom_bytes`.
 * * `rom_bytes` - The bytes of the N64 ROM in big endian format. It must have a minimum size of 0x101000 bytes.
 *
 * ## Return
 *
 * * `Ipl3Checksum_Error` indicating either a successful execution or the cause for failing.
 *   If execution fails then `dst_checksum0` and `dst_checksum1` are left untouched.
 */
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
