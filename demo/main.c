#include <stdio.h>
#include "caesium.h"

int main(int argc, char *argv[])
{
	if (argc != 3) {
		fprintf(stderr, "Wrong arguments.\nExiting.\n");
		return -1;
	}
	cs_image_pars options = initialize_parameters();
	cs_compress(argv[1], argv[2], &options);

	return 0;
}