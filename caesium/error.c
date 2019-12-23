#include <stdio.h>

#include "error.h"
#include "config.h"

int error_code = 0;

void display_error(error_level level, int code) {
    error_code = code;
    if (VERBOSE) {
        char *error_level = ((level) ? "[WARNING]" : "[ERROR]");

        fprintf(stderr, "%s %d: %s\n",
                error_level,
                code,
                get_error_message(code));
    }
}

const char *get_error_message(int code) {
    switch (code) {
        //Generic errors
        case 101:
            return "NULL file pointer while checking type.";
        case 103:
            return "File type not supported.";
        case 104:
            return "Could not open input file.";

        //JPEG related errors
        case 200:
            return "Failed to open JPEG file while trying to get markers";
        case 201:
            return "Failed to open input JPEG file while optimizing";
        case 202:
            return "Failed to open output JPEG file while optimizing";
        case 203:
            return "Failed to open JPEG file while compressing";
        case 204:
            return "Failed to open JPEG file while decompressing";
        case 205:
            return "Failed to retrieve input JPEG file size";
        case 206:
            return "Input JPEG file is too big";
        case 207:
            return "Compressor failed";
        case 208:
            return "Decompressor failed";
        case 209:
            return "CMYK images are not fully supported and can only be optimized.";

        //PNG related errors
        case 300:
            return "Failed to load PNG file.";
        case 301:
            return "Error while optimizing PNG.";
        case 303:
            return "Error while writing output PNG file.";
        case 304:
            return "Error while resizing PNG file.";
        case 305:
            return "PNG scaling factor must be a number greater than 0 and equal or minor to 1.";

        default:
            return "Unrecognized error.";
    }
}