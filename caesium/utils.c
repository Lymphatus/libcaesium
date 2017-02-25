#include <stdlib.h>
#include <stdio.h>

#include "utils.h"
#include "error.h"

image_type detect_image_type(FILE *pFile)
{
	unsigned char buffer[4];

	if (pFile == NULL) {
		display_error(WARNING, 101);
		return UNKN;
	}

	if (fread(buffer, 1, 4, pFile) < 4) {
		return UNKN;
	}

	if (buffer[0] == 0xFF && buffer[1] == 0xFF) {
		return CS_JPEG;
	} else if (buffer[0] == 0x89 && buffer[1] == 0x50) {
		return CS_PNG;
	} else if ((buffer[0] == 0x49 && buffer[1] == 0x49 && buffer[2] == 0x2A && buffer[3] == 0x00)
			   || (buffer[0] == 0x4D && buffer[1] == 0x4D && buffer[2] == 0x00 && buffer[3] == 0x2A)) {
		return CS_TIFF;
	}

	return UNKN;
}
