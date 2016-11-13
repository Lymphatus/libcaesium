#ifndef LIBCAESIUM_HELPER_H
#define LIBCAESIUM_HELPER_H

#include "cstypes.h"

void initialize_jpeg_parameters(cs_image_pars *options);

void initialize_png_parameters(cs_image_pars *options);

cs_image_pars initialize_parameters();

#endif
