/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "utils.h"

#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

uint8_t *read_binary_file(const char *path, size_t *size) {
    assert(path != NULL);
    assert(size != NULL);

    fprintf(stderr, "Reading file %s\n", path);

    FILE *f = fopen(path, "rb");
    if (f == NULL) {
        return NULL;
    }

    fseek(f, 0, SEEK_END);
    *size = ftell(f);
    fseek(f, 0, SEEK_SET);

    uint8_t *data = malloc(*size * sizeof(uint8_t));
    if (data == NULL) {
        fclose(f);
        return NULL;
    }

    size_t count = fread(data, sizeof(uint8_t), *size, f);
    if (count != *size) {
        free(data);
        fclose(f);
        return NULL;
    }

    fclose(f);
    return data;
}

uint32_t read_be_word(const uint8_t *src, size_t offset) {
    return (src[offset] << 24) | (src[offset+1] << 16) | (src[offset+2] << 8) | (src[offset+3] << 0);
}

const char *const ipl3checksum_error_str[] = {
    [Ipl3Checksum_Error_Okay] = "Okay",
    [Ipl3Checksum_Error_UnalignedRead] = "UnalignedRead",
    [Ipl3Checksum_Error_ByteConversion] = "ByteConversion",
    [Ipl3Checksum_Error_OutOfBounds] = "OutOfBounds",
    [Ipl3Checksum_Error_NullPointer] = "NullPointer",
    [Ipl3Checksum_Error_BufferNotBigEnough] = "BufferNotBigEnough",
    [Ipl3Checksum_Error_BufferSizeIsWrong] = "BufferSizeIsWrong",
    [Ipl3Checksum_Error_UnableToDetectCIC] = "UnableToDetectCIC",
    [Ipl3Checksum_Error_StringConversion] = "StringConversion",
};

const char *get_ipl3checksum_error_str(Ipl3Checksum_Error error) {
    return ipl3checksum_error_str[error.tag];
}

bool eprint_cickind_name(Ipl3Checksum_CICKind kind) {
    char *kind_name;
    Ipl3Checksum_Error kind_name_ok = ipl3checksum_cickind_get_name(kind, &kind_name);

    if (kind_name_ok.tag != Ipl3Checksum_Error_Okay) {
        fprintf(stderr, "Failed to get cic kind's name: %s\n", get_ipl3checksum_error_str(kind_name_ok));
        return false;
    }

    fprintf(stderr, "CIC kind name: '%s'\n", kind_name);

    ipl3checksum_free_string(kind_name);
    return true;
}
