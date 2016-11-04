#ifndef CS_CCLTYPES
#define CS_CCLTYPES

#include <stdbool.h>

typedef struct cs_jpeg_pars
{
	int quality;
	int color_space;
	int dct_method;
	bool exif_copy;
	bool lossless;
	//TODO uncomment when linking turbojpeg enum TJSAMP subsample;
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

	int width;
	int height;
	char **filepath;
} cs_image_pars;

enum image_type
{
	JPEG,
	PNG,
	UNKN,
};

#endif
