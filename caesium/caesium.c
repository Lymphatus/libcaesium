#include <stdlib.h>
#include <stdio.h>

#include "error.h"
#include "utils.h"
#include "png.h"
#include "jpeg.h"

bool cs_compress(const char *input_path, const char *output_path, cs_image_pars *options)
{
	FILE *pInputFile;
	image_type type;
	bool result = false;

	if ((pInputFile = fopen(input_path, "rb")) == NULL) {
		display_error(ERROR, 104);
		return result;
	}

	type = detect_image_type(pInputFile);

	fclose(pInputFile);

	if (type == UNKN) {
		display_error(WARNING, 103);
	} else if (type == JPEG) {
		if (options->jpeg.quality != 0) {
			cs_jpeg_compress(output_path, cs_jpeg_decompress(input_path, &options->jpeg), &options->jpeg);
			//The output is now the new input for optimization
			result = cs_jpeg_optimize(output_path, output_path, options->jpeg.exif_copy, input_path);
		} else {
			result = cs_jpeg_optimize(input_path, output_path, options->jpeg.exif_copy, input_path);
		}
	} else if (type == PNG) {
		result = cs_png_optimize(input_path, output_path, &options->png);
	}

	return result;
}

void initialize_jpeg_parameters(cs_image_pars *options)
{
	options->jpeg.quality = 0;
	options->jpeg.exif_copy = false;
	options->jpeg.dct_method = 2048;
}

void initialize_png_parameters(cs_image_pars *par)
{
	par->png.iterations = 10;
	par->png.iterations_large = 5;
	par->png.block_split_strategy = 4;
	par->png.lossy_8 = true;
	par->png.transparent = true;
	par->png.auto_filter_strategy = 1;
}

cs_image_pars initialize_parameters()
{
	cs_image_pars options;

	initialize_jpeg_parameters(&options);
	initialize_png_parameters(&options);

	return options;
}