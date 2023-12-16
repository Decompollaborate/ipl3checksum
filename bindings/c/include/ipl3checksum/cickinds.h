#ifndef IPL3CHECKSUM_CICKINDS_H
#define IPL3CHECKSUM_CICKINDS_H
#pragma once

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

#ifdef __cplusplus
}
#endif

#endif
