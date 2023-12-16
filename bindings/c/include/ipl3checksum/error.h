#ifndef IPL3CHECKSUM_ERROR_H
#define IPL3CHECKSUM_ERROR_H
#pragma once

#include <stdlib.h>

#ifdef __cplusplus
extern "C"
{
#endif

/* This needs to be synced with the Rust equivalent in `src/rs/error.rs` */
typedef enum Ipl3Checksum_Error_Tag {
    Ipl3Checksum_Error_Okay,
    Ipl3Checksum_Error_UnalignedRead,
    Ipl3Checksum_Error_ByteConversion,
    Ipl3Checksum_Error_OutOfBounds,
    Ipl3Checksum_Error_NullPointer,
    Ipl3Checksum_Error_BufferNotBigEnough,
    Ipl3Checksum_Error_BufferSizeIsWrong,
    Ipl3Checksum_Error_UnableToDetectCIC,
    Ipl3Checksum_Error_StringConversion,
} Ipl3Checksum_Error_Tag;

typedef struct Ipl3Checksum_Error {
    Ipl3Checksum_Error_Tag tag;
    union Ipl3Checksum_Error_Payload {
        struct Ipl3Checksum_Error_Payload_UnalignedRead {
            size_t offset;
        } UnalignedRead;
        struct Ipl3Checksum_Error_Payload_ByteConversion {
            size_t offset;
        } ByteConversion;
        struct Ipl3Checksum_Error_Payload_OutOfBounds {
            size_t offset;
            size_t requested_bytes;
            size_t buffer_len;
        } OutOfBounds;
        struct Ipl3Checksum_Error_Payload_BufferNotBigEnough {
            size_t buffer_len;
            size_t expected_len;
        } BufferNotBigEnough;
        struct Ipl3Checksum_Error_Payload_BufferSizeIsWrong {
            size_t buffer_len;
            size_t expected_len;
        } BufferSizeIsWrong;
    } payload;
} Ipl3Checksum_Error;

#ifdef __cplusplus
}
#endif

#endif
