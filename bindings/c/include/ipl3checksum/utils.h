#ifndef IPL3CHECKSUM_UTILS_H
#define IPL3CHECKSUM_UTILS_H
#pragma once

#include "error.h"

/**
 * Free a string returned by the ipl3checksum library.
 */
Ipl3Checksum_Error ipl3checksum_free_string(char *s);

#endif
