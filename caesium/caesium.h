#ifndef LIBCAESIUM_CAESIUM_H
#define LIBCAESIUM_CAESIUM_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>

typedef struct cs_jpeg_pars
{
	int quality;
	bool exif_copy;
	int dct_method;
	/*
	 * Parameters you have no reason to set as they will be
	 * overwritten during the process
	 */
	int color_space;
	int subsample;
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
	CS_JPEG,
	CS_PNG,
	UNKN,
} image_type;

typedef enum error_level
{
	ERROR = 0,
	WARNING = 1
} error_level;

bool cs_compress(const char *input_path, const char *output_path, cs_image_pars *options);
cs_image_pars initialize_parameters();

#ifdef __cplusplus
}
#endif

#endif