#ifndef LIBCAESIUM_JPEG_H
#define LIBCAESIUM_JPEG_H

#include <jpeglib.h>

#include "caesium.h"

bool cs_jpeg_optimize(const char *input_file, const char *output_file, bool exif, const char *exif_src);

struct jpeg_decompress_struct cs_get_markers(const char *input);

void cs_jpeg_compress(const char *output_file, unsigned char *image_buffer, cs_jpeg_pars *options);

unsigned char *cs_jpeg_decompress(const char *fileName, cs_jpeg_pars *options);

void jcopy_markers_execute(j_decompress_ptr srcinfo, j_compress_ptr dstinfo);

#endif //LIBCAESIUM_JPEG_H
