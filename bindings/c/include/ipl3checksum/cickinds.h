#ifndef IPL3CHECKSUM_CICKINDS_H
#define IPL3CHECKSUM_CICKINDS_H
#pragma once

#include <stddef.h>
#include <stdint.h>

#include "error.h"

#ifdef __cplusplus
extern "C"
{
#endif

/* This needs to be synced with the Rust equivalent in `src/rs/cickinds.rs` */
typedef enum Ipl3Checksum_CICKind {
    Ipl3Checksum_CICKind_CIC_6101,
    Ipl3Checksum_CICKind_CIC_6102_7101,
    Ipl3Checksum_CICKind_CIC_7102,
    Ipl3Checksum_CICKind_CIC_X103, // Both 6103 and 7103
    // 6104/7104 does not exist
    Ipl3Checksum_CICKind_CIC_X105, // Both 6105 and 7105
    Ipl3Checksum_CICKind_CIC_X106, // Both 6106 and 7106

    Ipl3Checksum_CICKind_CIC_5101, // Aleck 64
} Ipl3Checksum_CICKind;

uint32_t ipl3checksum_cickind_get_seed(Ipl3Checksum_CICKind self);

uint32_t ipl3checksum_cickind_get_magic(Ipl3Checksum_CICKind self);

/**
 * Returns the md5 hash for the specified CIC kind.
 *
 * If no errors happen (return is an `Ipl3Checksum_Error_Okay`), then the hash
 * is stored  on `dst_hash`.
 * This string is dynamically allocated by the library and it should be freed
 * (by passing it to `ipl3checksum_free_string`) to avoid memory leaks.
 */
Ipl3Checksum_Error ipl3checksum_cickind_get_hash_md5(Ipl3Checksum_CICKind self, char **dst_hash);

Ipl3Checksum_Error ipl3checksum_cickind_from_hash_md5(Ipl3Checksum_CICKind *kind_dst, const char *hash_str);

/**
 * Returns an human readable name for the specified CIC kind.
 *
 * If no errors happen (return is an `Ipl3Checksum_Error_Okay`), then the name
 * is stored  on `dst_name`.
 * This string is dynamically allocated by the library and it should be freed
 * (by passing it to `ipl3checksum_free_string`) to avoid memory leaks.
 */
Ipl3Checksum_Error ipl3checksum_cickind_get_name(Ipl3Checksum_CICKind self, char **dst_name);

Ipl3Checksum_Error ipl3checksum_cickind_from_name(Ipl3Checksum_CICKind *kind_dst, const char *name);

Ipl3Checksum_Error ipl3checksum_cickind_from_value(Ipl3Checksum_CICKind *kind_dst, size_t value);

#ifdef __cplusplus
}
#endif

#endif
