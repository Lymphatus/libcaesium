#include "caesium.h"
#include "helper.h"

int main()
{
	cs_image_pars options = initialize_parameters();
	cs_compress("test.jpg", "test1.jpg", &options);
	return 0;
}