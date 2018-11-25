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
	int compression_step_result;

	if ((pInputFile = fopen(input_path, "rb")) == NULL) {
		display_error(ERROR, 104);
		return result;
	}

	type = detect_image_type(pInputFile);

	fclose(pInputFile);

	if (type == UNKN) {
		display_error(WARNING, 103);
	} else if (type == CS_JPEG) {
		if (options->jpeg.quality != 0) {
			compression_step_result = cs_jpeg_compress(output_path, cs_jpeg_decompress(input_path, &options->jpeg), &options->jpeg);
			result = (bool) compression_step_result;
			//The output is now the new input for optimization
			if (result) {
				result = cs_jpeg_optimize(compression_step_result == 1 ? output_path : input_path, output_path, &options->jpeg, input_path);
			}
		} else {
			result = cs_jpeg_optimize(input_path, output_path, &options->jpeg, input_path);
		}
	} else if (type == CS_PNG) {
		result = cs_png_optimize(input_path, output_path, &options->png);
	}

	return result;
}

void initialize_jpeg_parameters(cs_image_pars *options)
{
	options->jpeg.quality = 0;
	options->jpeg.exif_copy = false;
	options->jpeg.dct_method = 2048;
	options->jpeg.scale_factor = 1.0;
}

void initialize_png_parameters(cs_image_pars *options)
{
	options->png.iterations = 2;
	options->png.iterations_large = 1;
	options->png.block_split_strategy = 0;
	options->png.lossy_8 = true;
	options->png.transparent = true;
	options->png.auto_filter_strategy = true;
	options->png.scale_factor = 1.0;
}

cs_image_pars initialize_parameters()
{
	cs_image_pars options;

	initialize_jpeg_parameters(&options);
	initialize_png_parameters(&options);

	return options;
}
