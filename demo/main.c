#include <stdio.h>
#include <stdlib.h>
#include "../caesium/caesium.h"
#include "config.h"

int main(int argc, char *argv[])
{
	fprintf(stdout, "libcaesium demo application v%d.%d.%d\n",
		   VERSION_MAJOR,
		   VERSION_MINOR,
		   VERSION_PATCH);

	if (argc != 3) {
		fprintf(stderr, "Wrong arguments.\nExiting.\n");
		exit(EXIT_FAILURE);
	}

	cs_image_pars options = initialize_parameters();
	int error_code = 0;
	cs_compress(argv[1], argv[2], &options, &error_code);

	exit(error_code);
}