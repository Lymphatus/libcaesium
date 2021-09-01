#define STB_IMAGE_RESIZE_IMPLEMENTATION
#define STB_IMAGE_IMPLEMENTATION
#define STB_IMAGE_WRITE_IMPLEMENTATION

#define STBIW_ASSERT(x)

#include <stdlib.h>
#include <zopflipng_lib.h>

#include "vendor/stb_image.h"
#include "vendor/stb_image_write.h"
#include "vendor/stb_image_resize.h"
#include "png.h"
#include "vendor/lodepng.h"
#include "error.h"

bool cs_png_optimize(const char *input, const char *output, cs_png_pars *options)
{
	bool result = false;
	CZopfliPNGOptions png_options;
	int error_code = 0;

	if (options->scale_factor > 0.0 && options->scale_factor < 1.0) {
		result = cs_png_resize(input, output, options->scale_factor);
		if (!result) {
			libcaesium_display_error(ERROR, 304);
			return result;
		}
	} else if (options->scale_factor != 1.0) {
		libcaesium_display_error(ERROR, 305);
		return false;
	}

	CZopfliPNGSetDefaults(&png_options);

	unsigned char *orig_buffer;
	size_t orig_buffer_size;

	unsigned char *resultpng = NULL;
	size_t resultpng_size;

	png_options.num_iterations = options->iterations;
	png_options.num_iterations_large = options->iterations_large;
	png_options.block_split_strategy = options->block_split_strategy;

	png_options.lossy_8bit = options->lossy_8;
	png_options.lossy_transparent = options->transparent;

	png_options.auto_filter_strategy = options->auto_filter_strategy;

	if (lodepng_load_file(&orig_buffer, &orig_buffer_size, options->scale_factor == 1 ? input : output) != 0) {
		error_code = 300;
		goto cleanup;
	}

	int code = CZopfliPNGOptimize(orig_buffer,
								  orig_buffer_size,
								  &png_options,
								  0,
								  &resultpng,
								  &resultpng_size);

	if (code != 0) {
		error_code = 301;
		goto cleanup;
	}

	if (lodepng_save_file(resultpng, resultpng_size, output) != 0) {
		error_code = 302;
		goto cleanup;
	}

	result = true;

	cleanup:
	free(orig_buffer);
	free(resultpng);
	if (error_code != 0) {
		libcaesium_display_error(ERROR, error_code);
	}
	return result;
}

bool cs_png_resize(const char *input, const char *output, double factor)
{
	unsigned char *input_pixels;
	unsigned char *output_pixels;
	int w, h;
	int n;
	int out_w, out_h;
	int result;

	input_pixels = stbi_load(input, &w, &h, &n, 0);
	if (input_pixels == 0) {
		return false;
	}
	out_w = (int) round(w * factor);
	out_h = (int) round(h * factor);

	output_pixels = (unsigned char *) malloc(out_w * out_h * n);
	result = stbir_resize_uint8(input_pixels, w, h, 0, output_pixels, out_w, out_h, 0, n);
	if (!result) {
		free(output_pixels);
		return false;
	}
	result = stbi_write_png(output, out_w, out_h, n, output_pixels, 0);
	if (!result) {
		return false;
	}

	return true;
}
