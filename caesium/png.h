#ifndef LIBCAESIUM_PNG_H
#define LIBCAESIUM_PNG_H

#include "caesium.h"

bool cs_png_optimize(const char *input, const char *output, cs_png_pars *options);

bool cs_png_resize(const char *input, const char *output, double factor);

#endif //LIBCAESIUM_PNG_H
