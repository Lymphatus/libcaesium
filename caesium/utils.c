#include <stdlib.h>
#include <stdio.h>

#include "utils.h"
#include "error.h"

enum image_type detect_image_type(FILE *pFile)
{
	unsigned char buffer[2];

	if (pFile == NULL) {
		display_error(0, 1);
		return UNKN;
	}

	if (fread(buffer, 1, 2, pFile) < 2) {
		display_error(0, 2);
		return UNKN;
	}

	if (buffer[0] == 0xFF && buffer[1] == 0xD8) {
		return JPEG;
	} else if (buffer[0] == 0x89 && buffer[1] == 0x50) {
		return PNG;
	}

	return UNKN;
}
