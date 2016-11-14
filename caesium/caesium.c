#include <stdlib.h>
#include <stdio.h>

#include "cstypes.h"
#include "error.h"
#include "utils.h"
#include "caesium.h"
#include "png.h"
#include "jpeg.h"

bool cs_compress(const char *inputPath, const char *outputPath, cs_image_pars *options)
{
	FILE *pInputFile;
	image_type type;
	bool result = false;

	//Inline... Should I split into 2 lines?
	if ((pInputFile = fopen(inputPath, "rb")) == NULL) {
		display_error(ERROR, 4);
		return result;
	}

	type = detect_image_type(pInputFile);

	fclose(pInputFile);

	if (type == UNKN) {
		display_error(WARNING, 3);
	} else if (type == JPEG) {
		if (options->jpeg.quality != 0) {
			cs_jpeg_compress(outputPath, cs_jpeg_decompress(inputPath, &options->jpeg), &options->jpeg);
			inputPath = outputPath;
		}
		cs_jpeg_optimize(inputPath, outputPath, options->jpeg.exif_copy, inputPath);
	} else if (type == PNG) {
		result = cs_png_optimize(inputPath, outputPath, &options->png);
	}

	return result;
}