#ifndef LIBCAESIUM_CAESIUM_H
#define LIBCAESIUM_CAESIUM_H

#include <stdbool.h>

#include "cstypes.h"

bool cs_compress(const char *input, const char *output, cs_image_pars *options);

#endif