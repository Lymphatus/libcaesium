#include <stdlib.h>
#include <stdio.h>

#include "cstypes.h"
#include "error.h"
#include "utils.h"
#include "caesium.h"

bool cs_compress(const char *input, const char *output, cs_image_pars *options)
{
	FILE *pInputFile;
	enum image_type type;
	bool result = false;

	//Inline... Should I split into 2 lines?
	if ((pInputFile = fopen(input, "rb")) == NULL) {
		display_error(0, 4);
		return result;
	}

	type = detect_image_type(pInputFile);

	fclose(pInputFile);

	if (type == UNKN) {
		display_error(1, 3);
	} else if (type == JPEG) {
		//TODO result Compress JPEG
		printf("Called JPEG compression\n");
	} else if (type == PNG) {
		//TODO result Compress PNG
		printf("Called PNG compression\n");
	}

	return result;
}