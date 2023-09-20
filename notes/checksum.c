/* SPDX-License-Identifier: CC0-1.0 */

#include <stdint.h>

uint32_t readWord(const uint8_t *rom , uint32_t offset) {
    return (rom[offset] << 24) | (rom[offset+1] << 16) | (rom[offset+2] << 8) | (rom[offset+3]);
}

uint32_t getSeed(uint32_t cic) {
    switch (cic) {
        case 6101:
        case 6102:
        case 7101:
        case 7102:
        default:
            return 0x3F;

        case 6103:
        case 7103:
            return 0x78;

        case 6105:
        case 7105:
            return 0x91;

        case 6106:
        case 7106:
            return 0x85;
    }
}

uint32_t getMagic(uint32_t cic) {
    switch (cic) {
        case 6101:
        case 6102:
        case 7101:
        case 7102:
        case 6105:
        case 7105:
        default:
            return 0x5D588B65;

        case 6103:
        case 7103:
        case 6106:
        case 7106:
            return 0x6C078965;
    }
}

void calculateChecksum(const uint8_t *rom, uint32_t cic, uint32_t *dst1, uint32_t *dst2) {
    uint32_t seed = getSeed(cic);
    uint32_t magic = getMagic(cic);
    uint32_t at;
    uint32_t v0, v1;
    uint32_t a0, a1, a2, a3;
    uint32_t t0, t1, t2, t3, t4, t5, t6, t7, t8, t9;
    uint32_t s0, s6, ra, lo;
    uint32_t entrypointRam;

    s6 = seed;

    a0 = readWord(rom, 8);
    switch (cic) {
        case 6103:
        case 7103:
            a0 -= 0x100000;
            break;

        case 6106:
        case 7106:
            a0 -= 0x200000;
            break;
    }

    entrypointRam = a0;

    at = magic;
    lo = s6 * at;

    switch (cic) {
        case 6105:
        case 7105:
            s6 = 0xA0000200;
            break;
    }

    ra = 0x100000;

    v1 = 0;
    t0 = 0;
    t1 = a0;

    t5 = 0x20;

    v0 = lo;
    v0 += 1;

    a3 = v0;
    t2 = v0;
    t3 = v0;
    s0 = v0;
    a2 = v0;
    t4 = v0;

    do {
        v0 = readWord(rom, t1 - entrypointRam + 0x1000);
        v1 = a3 + v0;

        at = v1 < a3;

        a1 = v1;

        if (at != 0) {
            t2 = t2 + 0x1;
        }

        v1 = v0 & 0x1F;
        t7 = t5 - v1;

        t8 = v0 >> t7;
        t6 = v0 << v1;

        a0 = t6 | t8;
        at = a2 < v0;
        a3 = a1;

        t3 = t3 ^ v0;

        s0 = s0 + a0;

        if (at != 0) {
            t9 = a3 ^ v0;
            a2 = t9 ^ a2;
        } else {
            a2 = a2 ^ a0;
        }

        switch (cic) {
            case 6105:
            case 7105:
                /* ipl3 6105 copies 0x330 bytes from the ROM's offset 0x000554 (or offset 0x000514 into IPL3) to vram 0xA0000004 */
                t7 = readWord(rom, s6 - 0xA0000004 + 0x000554);

                t0 = t0 + 0x4;
                s6 = s6 + 0x4;
                t7 = v0 ^ t7;

                t4 = t7 + t4;

                t7 = 0xA00002FF;

                t1 = t1 + 0x4;

                s6 = s6 & t7;
                break;

            default:
                t0 = t0 + 0x4;
                t7 = v0 ^ s0;
                t1 = t1 + 0x4;
                t4 = t7 + t4;
                break;
        }
    } while (t0 != ra);

    switch (cic) {
        case 6103:
        case 7103:
            t6 = a3 ^ t2;
            a3 = t6 + t3;
            t8 = s0 ^ a2;
            s0 = t8 + t4;
            break;

        case 6106:
        case 7106:
            t6 = a3 * t2;
            a3 = t6 + t3;
            t8 = s0 * a2;
            s0 = t8 + t4;
            break;

        default:
            t6 = a3 ^ t2;
            a3 = t6 ^ t3;
            t8 = s0 ^ a2;
            s0 = t8 ^ t4;
            break;
    }

    *dst1 = a3;
    *dst2 = s0;
}

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    uint8_t rom[0x101000];
    FILE *file;
    uint32_t w0;
    uint32_t w1;
    size_t readlen;
    uint32_t cic;

    if (argc < 3) {
        fprintf(stderr, "Usage: %s <rom_path> <cic_type>\n", argv[0]);
        return 1;
    }

    file = fopen(argv[1], "rb");
    if (file == NULL) {
        fprintf(stderr, "Not able to open rom file '%s'\n", argv[1]);
        return 1;
    }

    readlen = fread(rom, sizeof(uint8_t), 0x101000, file);
    if (readlen != 0x101000) {
        fprintf(stderr, "Not able to read rom file\n");
        return 1;
    }

    {
        char *str_end;

        cic = strtol(argv[2], &str_end, 10);
        if (argv[2] == str_end) {
            fprintf(stderr, "Not able to parse cic argument (%s)\n", argv[2]);
            return 1;
        }
    }

    calculateChecksum(rom, cic, &w0, &w1);

    printf("0x%08X\n0x%08X\n", w0, w1);

    fclose(file);

    return 0;
}
