#include <stdio.h>
#include <jpeglib.h>
#include <string.h>
#include <turbojpeg.h>
#include <limits.h>

#include "jpeg.h"
#include "error.h"

struct jpeg_decompress_struct cs_get_markers(const char *input)
{
	FILE *fp;
	struct jpeg_decompress_struct einfo;
	struct jpeg_error_mgr eerr;
	einfo.err = jpeg_std_error(&eerr);

	jpeg_create_decompress(&einfo);

	//Check for errors
	if ((fp = fopen(input, "rb")) == NULL) {
		display_error(ERROR, 200);
	}

	//Create the IO instance for the input file
	jpeg_stdio_src(&einfo, fp);

	//Save EXIF info
	for (int m = 0; m < 16; m++) {
		jpeg_save_markers(&einfo, JPEG_APP0 + m, 0xFFFF);
	}

	jpeg_read_header(&einfo, true);

	fclose(fp);

	return einfo;
}

bool cs_jpeg_optimize(const char *input_file, const char *output_file, cs_jpeg_pars *options, const char *exif_src)
{
	//File pointer for both input and output
	FILE *fp;

	//Those will hold the input/output structs
	struct jpeg_decompress_struct srcinfo;
	struct jpeg_compress_struct dstinfo;

	//Error handling
	struct jpeg_error_mgr jsrcerr, jdsterr;

	//Input/Output array coefficents
	jvirt_barray_ptr *src_coef_arrays;
	jvirt_barray_ptr *dst_coef_arrays;

	//Set errors and create the compress/decompress istances
	srcinfo.err = jpeg_std_error(&jsrcerr);
	jpeg_create_decompress(&srcinfo);
	dstinfo.err = jpeg_std_error(&jdsterr);
	jpeg_create_compress(&dstinfo);

	//Check for errors
	if ((fp = fopen(input_file, "rb")) == NULL) {
		display_error(ERROR, 201);
	}

	//Create the IO instance for the input file
	jpeg_stdio_src(&srcinfo, fp);

	//Save EXIF info
	if (options->exif_copy) {
		for (int m = 0; m < 16; m++) {
			jpeg_save_markers(&srcinfo, JPEG_APP0 + m, 0xFFFF);
		}
	}

	//Read the input headers
	(void) jpeg_read_header(&srcinfo, true);


	//Read input coefficents
	src_coef_arrays = jpeg_read_coefficients(&srcinfo);

	//Copy parameters
	jpeg_copy_critical_parameters(&srcinfo, &dstinfo);

	//Set coefficents array to be the same
	dst_coef_arrays = src_coef_arrays;

	//We don't need the input file anymore
	fclose(fp);

	//Check for errors
	if ((fp = fopen(output_file, "wb")) == NULL) {
		display_error(ERROR, 202);
	}

	//CRITICAL - This is the optimization step
	dstinfo.optimize_coding = true;

	//Set the output file parameters
	jpeg_stdio_dest(&dstinfo, fp);

	//Actually write the coefficients
	jpeg_write_coefficients(&dstinfo, dst_coef_arrays);

	//Write EXIF
	if (options->exif_copy) {
		if (strcmp(input_file, exif_src) == 0) {
			jcopy_markers_execute(&srcinfo, &dstinfo);
		} else {
			//For standard compression EXIF data
			struct jpeg_decompress_struct einfo = cs_get_markers(exif_src);
			jcopy_markers_execute(&einfo, &dstinfo);
			jpeg_destroy_decompress(&einfo);
		}
	}

	//Finish and free
	jpeg_finish_compress(&dstinfo);
	jpeg_destroy_compress(&dstinfo);
	(void) jpeg_finish_decompress(&srcinfo);
	jpeg_destroy_decompress(&srcinfo);

	//Close the output file
	fclose(fp);

	return true;
}

bool cs_jpeg_compress(const char *output_file, unsigned char *image_buffer, cs_jpeg_pars *options)
{
	FILE *fp;
	tjhandle tjCompressHandle;
	unsigned char *output_buffer;
	unsigned long output_size = 0;
	output_buffer = NULL;
	int result = 0;

	//Check for errors
	if ((fp = fopen(output_file, "wb")) == NULL) {
		display_error(ERROR, 203);
	}

	tjCompressHandle = tjInitCompress();

	result = tjCompress2(tjCompressHandle,
				image_buffer,
				options->width,
				0,
				options->height,
				options->color_space,
				&output_buffer,
				&output_size,
				options->subsample,
				options->quality,
				options->dct_method);
	if (result == -1) {
		display_error(ERROR, 207);
	} else {
		fwrite(output_buffer, output_size, 1, fp);
	}

	fclose(fp);
	tjDestroy(tjCompressHandle);
	tjFree(output_buffer);
	tjFree(image_buffer);

	return true;
}

unsigned char *cs_jpeg_decompress(const char *fileName, cs_jpeg_pars *options)
{
	FILE *fp;
	long sourceJpegBufferSize = 0;
	unsigned char *sourceJpegBuffer = NULL;
	tjhandle tjDecompressHandle;
	int fileWidth = 0, fileHeight = 0, jpegSubsamp = 0, colorSpace = 0, result = 0;

	if ((fp = fopen(fileName, "rb")) == NULL) {
		display_error(ERROR, 204);
	}
	fseek(fp, 0, SEEK_END);
	sourceJpegBufferSize = ftell(fp);
	if (sourceJpegBufferSize == -1) {
		display_error(ERROR, 205);
	}
	if (sourceJpegBufferSize > INT_MAX) {
		display_error(ERROR, 206);
	}
	sourceJpegBuffer = tjAlloc((int) sourceJpegBufferSize);

	fseek(fp, 0, SEEK_SET);
	fread(sourceJpegBuffer, (size_t) sourceJpegBufferSize, 1, fp);
	tjDecompressHandle = tjInitDecompress();
	tjDecompressHeader3(tjDecompressHandle, sourceJpegBuffer, (unsigned long) sourceJpegBufferSize, &fileWidth, &fileHeight,
						&jpegSubsamp, &colorSpace);

	options->width = fileWidth;
	options->height = fileHeight;

	options->subsample = (enum TJSAMP) jpegSubsamp;
	options->color_space = colorSpace;

	unsigned char *temp = tjAlloc(options->width * options->height * tjPixelSize[options->color_space]);

	result = tjDecompress2(tjDecompressHandle,
				  sourceJpegBuffer,
				  (unsigned long) sourceJpegBufferSize,
				  temp,
				  options->width,
				  0,
				  options->height,
				  options->color_space,
				  options->dct_method);
	if (result == -1) {
		display_error(ERROR, 208);
	}

	tjDestroy(tjDecompressHandle);
	tjFree(sourceJpegBuffer);

	return temp;
}

void jcopy_markers_execute(j_decompress_ptr srcinfo, j_compress_ptr dstinfo)
{
	jpeg_saved_marker_ptr marker;

	for (marker = srcinfo->marker_list; marker != NULL; marker = marker->next) {
		if (dstinfo->write_JFIF_header &&
			marker->marker == JPEG_APP0 &&
			marker->data_length >= 5 &&
			GETJOCTET(marker->data[0]) == 0x4A &&
			GETJOCTET(marker->data[1]) == 0x46 &&
			GETJOCTET(marker->data[2]) == 0x49 &&
			GETJOCTET(marker->data[3]) == 0x46 &&
			GETJOCTET(marker->data[4]) == 0)
			continue;                 /* reject duplicate JFIF */
		if (dstinfo->write_Adobe_marker &&
			marker->marker == JPEG_APP0 + 14 &&
			marker->data_length >= 5 &&
			GETJOCTET(marker->data[0]) == 0x41 &&
			GETJOCTET(marker->data[1]) == 0x64 &&
			GETJOCTET(marker->data[2]) == 0x6F &&
			GETJOCTET(marker->data[3]) == 0x62 &&
			GETJOCTET(marker->data[4]) == 0x65)
			continue;                 /* reject duplicate Adobe */
		jpeg_write_marker(dstinfo, marker->marker,
						  marker->data, marker->data_length);
	}
}