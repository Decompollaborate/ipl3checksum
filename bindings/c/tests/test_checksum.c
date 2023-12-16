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

    if (argc < 2) {
        print_usage(argc, argv);
        return -1;
    }

    const char *bin_path = argv[1];
    const char *cic_kind_name = argv[2];

    size_t bin_size = 0;
    uint8_t *bin = read_binary_file(bin_path, &bin_size);
    assert(bin_size > 0);
    assert(bin != NULL);

    fprintf(stderr, "CIC kind: '%s'\n", cic_kind_name);
    // TODO: Don't hardcode
    Ipl3Checksum_CICKind kind = Ipl3Checksum_CICKind_CIC_6102_7101;

    {
        uint32_t checksum0;
        uint32_t checksum1;

        Ipl3Checksum_Error err = ipl3checksum_calculate_checksum(&checksum0, &checksum1, bin_size, bin, kind);

        if (err.tag == Ipl3Checksum_Error_Okay) {
            fprintf(stderr, "Computed checksum: %08X %08X\n", checksum0, checksum1);
        } else {
            fprintf(stderr, "Error trying to compute the checksum: %s\n", get_ipl3checksum_error_str(err));
        }
    }

    free(bin);

    return ret;
}
