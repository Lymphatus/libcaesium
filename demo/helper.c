#include <stdbool.h>
#include <turbojpeg.h>

#include "helper.h"

void initialize_jpeg_parameters(cs_image_pars *options)
{
	options->jpeg.quality = 0;
	options->jpeg.exif_copy = false;
	options->jpeg.dct_method = TJFLAG_FASTDCT;
	options->jpeg.width = 0;
	options->jpeg.height = 0;
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
