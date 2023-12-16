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
} Ipl3Checksum_CICKind;

uint32_t ipl3checksum_cickind_get_seed(Ipl3Checksum_CICKind self);

uint32_t ipl3checksum_cickind_get_magic(Ipl3Checksum_CICKind self);

// const char *ipl3checksum_cickind_get_hash_md5(Ipl3Checksum_CICKind self);

Ipl3Checksum_Error ipl3checksum_cickind_from_hash_md5(Ipl3Checksum_CICKind *kind_dst, const char *hash_str);

// const char *ipl3checksum_cickind_get_name(Ipl3Checksum_CICKind self);

Ipl3Checksum_Error ipl3checksum_cickind_from_name(Ipl3Checksum_CICKind *kind_dst, const char *name);

Ipl3Checksum_Error ipl3checksum_cickind_from_value(Ipl3Checksum_CICKind *kind_dst, size_t value);

#ifdef __cplusplus
}
#endif

#endif
