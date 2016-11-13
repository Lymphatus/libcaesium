#include <stdio.h>

#include "error.h"

void display_error(error_level level, int code)
{
	char *error_level = ((level) ? "WARNING" : "ERROR");
	fprintf(stderr, "%s %d: %s\n",
			error_level,
			code,
			get_error_message(code));
}

const char *get_error_message(int code)
{
	switch (code) {
		//Generic errors
		case 1:
			return "NULL file pointer while checking type.";
		case 2:
			return "Could not read enough file bytes for type checking.";
		case 3:
			return "File type not supported.";
		case 4:
			return "Could not open input file.";

			//PNG related errors
		case 200:
			return "Failed to load PNG file.";
		case 201:
			return "Error while optimizing PNG.";
		case 203:
			return "Error while writing output PNG file.";

		default:
			return "Unrecognized error.";
	}
}