/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "ipl3checksum.h"

#include <assert.h>
#include <stdio.h>

#include "utils.h"

void print_usage(int argc, char *argv[]) {
    (void)argc;

    fprintf(stderr, "Usage: %s bin_file cic_kind\n", argv[0]);
    fprintf(stderr, "\n");
    fprintf(stderr, "This programs computes the ipl3 checksum of a big endian binary file with a given cic kind\n");
}

int main(int argc, char *argv[]) {
    int ret = 0;

    if (argc < 3) {
        print_usage(argc, argv);
        return -1;
    }

    const char *bin_path = argv[1];
    const char *cic_kind_name = argv[2];

    size_t bin_size = 0;
    uint8_t *bin = read_binary_file(bin_path, &bin_size);
    assert(bin_size > 0);
    assert(bin != NULL);

    fprintf(stderr, "Passed CIC kind: '%s'\n", cic_kind_name);
    Ipl3Checksum_CICKind kind;
    {
        Ipl3Checksum_Error err = ipl3checksum_cickind_from_name(&kind, cic_kind_name);

        if (err.tag == Ipl3Checksum_Error_Okay) {
            char *kind_name;
            Ipl3Checksum_Error kind_name_ok = ipl3checksum_cickind_get_name(kind, &kind_name);

            if (kind_name_ok.tag != Ipl3Checksum_Error_Okay) {
                fprintf(stderr, "Failed to get cic kind's name: %s\n", get_ipl3checksum_error_str(kind_name_ok));
                ret++;
                goto cleanup;
            }

            fprintf(stderr, "Parsed kind: '%s'\n", kind_name);

            ipl3checksum_free_string(kind_name);
        } else {
            fprintf(stderr, "Passed CIC kind was not valid: %s\n", get_ipl3checksum_error_str(err));
            ret++;
            goto cleanup;
        }
    }

    uint32_t expected_checksum0 = read_be_word(bin, 0x10);
    uint32_t expected_checksum1 = read_be_word(bin, 0x14);

    fprintf(stderr, "Expected checksum: %08X %08X\n", expected_checksum0, expected_checksum1);

    {
        uint32_t checksum0;
        uint32_t checksum1;

        Ipl3Checksum_Error err = ipl3checksum_calculate_checksum(&checksum0, &checksum1, bin_size, bin, kind);

        if (err.tag == Ipl3Checksum_Error_Okay) {
            fprintf(stderr, "Computed checksum: %08X %08X\n", checksum0, checksum1);
            if ((checksum0 == expected_checksum0) && (checksum1 == expected_checksum1)) {
                fprintf(stderr, "Checksum matches\n");
            } else {
                fprintf(stderr, "Checksum doesn't match\n");
                ret++;
            }
        } else {
            fprintf(stderr, "Error trying to compute the checksum: %s\n", get_ipl3checksum_error_str(err));
            ret++;
        }
    }

cleanup:
    free(bin);

    return ret;
}
