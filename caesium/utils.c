#include <stdlib.h>
#include <stdio.h>

#include "utils.h"
#include "error.h"

image_type detect_image_type(FILE *pFile)
{
	unsigned char buffer[2];

	if (pFile == NULL) {
		libcaesium_display_error(WARNING, 101);
		return UNKN;
	}

	if (fread(buffer, 1, 2, pFile) < 2) {
		return UNKN;
	}

	if (buffer[0] == 0xFF && buffer[1] == 0xD8) {
		return CS_JPEG;
	} else if (buffer[0] == 0x89 && buffer[1] == 0x50) {
		return CS_PNG;
	}

	return UNKN;
}
