#include "tiff.h"


bool cs_tiff_optimize(const char *input, const char *output, cs_tiff_pars* options)
{
	TIFF* in;
	TIFF* out;

	in = TIFFOpen(input, "rb");

	TIFFClose(in);
	TIFFClose(out);

	return true;
}