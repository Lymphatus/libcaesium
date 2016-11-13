#ifndef LIBCAESIUM_CSTYPES_H
#define LIBCAESIUM_CSTYPES_H

#include <stdbool.h>
#include <turbojpeg.h>

typedef struct cs_jpeg_pars
{
	int quality;
	int color_space;
	int dct_method;
	bool exif_copy;
	enum TJSAMP subsample;
	int width;
	int height;
} cs_jpeg_pars;

typedef struct cs_png_pars
{
	int iterations;
	int iterations_large;
	int block_split_strategy;
	bool lossy_8;
	bool transparent;
	int auto_filter_strategy;
} cs_png_pars;

typedef struct cs_image_pars
{
	cs_jpeg_pars jpeg;
	cs_png_pars png;
} cs_image_pars;

typedef enum image_type
{
	JPEG,
	PNG,
	UNKN,
} image_type;

typedef enum error_level
{
	ERROR = 0,
	WARNING = 1
} error_level;

#endif
