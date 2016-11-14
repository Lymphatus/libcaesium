# libcaesium
[![Build Status](https://travis-ci.org/Lymphatus/libcaesium.svg?branch=master)](https://travis-ci.org/Lymphatus/libcaesium)  

Libcaesium is a simple library performing JPEG and PNG compression/optimization using [mozjpeg](https://github.com/mozilla/mozjpeg) and [zopfli](https://github.com/google/zopfli).


## Basic usage

Libcaesium exposes one single function, auto-detecting the input file type:
```C
bool cs_compress(const char *input,
                 const char *output,
                 cs_image_pars *options);
```
#### Parameters
**input** - input file path  
**output** - output file path  
**options** - pointer to the options struct, containing compression parameters (see below)  

#### Return value
**true** if the compression has successfully ended, or **false** if any error occurs.

## Compression options
Libcaesium supports a few compression parameters for each JPEG and PNG.
They are defined into a top level struct containing each supported file parameters, as follows:
```C
typedef struct cs_image_pars
{
	cs_jpeg_pars jpeg;
	cs_png_pars png;
} cs_image_pars;
```
### JPEG
```C
typedef struct cs_jpeg_pars
{
	int quality;
	bool exif_copy;
	int dct_method;
	/*
	 * Parameters you have no reason to set as they will be
	 * overwritten during the process
	 */
	int color_space;
	enum TJSAMP subsample;
	int width;
	int height;
} cs_jpeg_pars;
```
The first 3 parameters matters, in term of compression, while the others will be set by the compressor/decompressor
during the compression progress and thus they will be overwritten.
- **quality**: in a range from 0 to 100, the quality of the resulting image. **Note** that 0 means _optimization_ (see below). Default: 65.
- **exif_copy**: set it to _true_ to copy EXIF tag info after compression. Default: false;
- **dct_method**: one of the turbojpeg DCT flags. Default: TJFLAG_FASTDCT.

### PNG
```C
typedef struct cs_png_pars
{
	int iterations;
	int iterations_large;
	int block_split_strategy;
	bool lossy_8;
	bool transparent;
	int auto_filter_strategy;
} cs_png_pars;
```
Those are the zopflipng compression parameters.
- **iterations**: number of iterations (more means more compression). Default: 10.
- **iteration_large**: number of iterations for large files. Default: 5.
- **block_split_strategy**: filter strategy. Default: 4;
- **lossy_8**: convert 16-bit per channel image to 8-bit per channel. Default: true.
- **transparent**: remove colors behind alpha channel 0. Default: true.
- **auto_filter_strategy**: legacy.

## Compilation
TODO

## Installation
TODO

## Compression vs Optimization
JPEG is a lossy format: that means you will always lose some information after each compression. So, compressing a file with
100 quality for 10 times will result in a always different image, even though you can't really see the difference.
Libcaesium also supports optimization, by setting the _quality_ to 0. This performs a lossy process, resulting in the same image,
but with a smaller size (10-15% usually).  
PNG is lossless, so libcaesium will always perform optimization rather than compression.