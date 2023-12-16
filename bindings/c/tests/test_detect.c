/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "ipl3checksum.h"

#include <assert.h>
#include <stdio.h>

#include "utils.h"

void print_usage(int argc, char *argv[]) {
    (void)argc;

    fprintf(stderr, "Usage: %s bin_file\n", argv[0]);
    fprintf(stderr, "\n");
    fprintf(stderr, "This programs detects the cic kind of a given big endian rom\n");
}

int main(int argc, char *argv[]) {
    int ret = 0;

    if (argc < 1) {
        print_usage(argc, argv);
        return -1;
    }

    const char *bin_path = argv[1];

    size_t bin_size = 0;
    uint8_t *bin = read_binary_file(bin_path, &bin_size);
    assert(bin_size > 0);
    assert(bin != NULL);

    {
        Ipl3Checksum_CICKind kind;

        Ipl3Checksum_Error err = ipl3checksum_detect_cic(&kind, bin_size, bin);

        if (err.tag == Ipl3Checksum_Error_Okay) {
            // TODO: use string name instead
            fprintf(stderr, "Detected CIC kind: %i\n", kind);
        } else {
            fprintf(stderr, "Error trying to detect the cic: %s\n", get_ipl3checksum_error_str(err));
        }
    }

    free(bin);

    return ret;
}
