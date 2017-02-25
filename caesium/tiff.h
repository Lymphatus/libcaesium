#include <stdbool.h>

#ifndef LIBCAESIUM_TIFF_H
#define LIBCAESIUM_TIFF_H

#include <tiffio.h>
#include "caesium.h"

bool cs_tiff_optimize(const char *input, const char *output, cs_tiff_pars* options);

#endif //LIBCAESIUM_TIFF_H
