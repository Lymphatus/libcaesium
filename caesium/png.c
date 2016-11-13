//
// Created by Matteo Paonessa on 08/11/16.
//

#include <stdlib.h>
#include <zopflipng/zopflipng_lib.h>

#include "png.h"
#include "lodepng.h"
#include "error.h"

bool cs_png_optimize(const char *input, const char *output, cs_png_pars *options)
{
	bool result = false;
	CZopfliPNGOptions png_options;

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

	if (lodepng_load_file(&orig_buffer, &orig_buffer_size, input) != 0) {
		display_error(ERROR, 200);
		goto cleanup;
	}

	if (CZopfliPNGOptimize(orig_buffer,
						   orig_buffer_size,
						   &png_options,
						   0,
						   &resultpng,
						   &resultpng_size) != 0) {
		display_error(ERROR, 201);
		goto cleanup;
	}

	if (lodepng_save_file(resultpng, resultpng_size, output) != 0) {
		display_error(ERROR, 202);
		goto cleanup;
	}

	result = true;

	cleanup:
	free(orig_buffer);
	free(resultpng);
	return result;
}
