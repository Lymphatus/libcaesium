# libcaesium
[![Build Status](https://travis-ci.org/Lymphatus/libcaesium.svg?branch=master)](https://travis-ci.org/Lymphatus/libcaesium)  

Libcaesium is a simple library performing JPEG and PNG compression/optimization using [mozjpeg](https://github.com/mozilla/mozjpeg) and [zopfli](https://github.com/google/zopfli).

## Download
Binaries not available yet. Please refer to the compilation section below.

## Basic usage

Libcaesium exposes one single function to compress, auto-detecting the input file type:
```C
bool cs_compress(const char *input,
                 const char *output,
                 cs_image_pars *options,
                 int* err_n);
```
#### Parameters
**input** - input file path  
**output** - output file path  
**options** - pointer to the options struct, containing compression parameters (see below)  
**err_n** - pointer to an integer that will contain the error code if something went wrong during compression 

#### Return value
**true** if the compression has successfully ended, or **false** if any error occurs. If any error occurred, the **err_n**
variable will contain the error code. See `error.h` for further info.

## Compression options
Libcaesium supports a few compression parameters for each JPEG and PNG.
You need to initialize the default values before compressing by calling `initialize_parameters()`.  

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
	double scale_factor;
} cs_jpeg_pars;
```
The first 4 parameters matters, in term of compression, while the others will be set by the compressor/decompressor
during the compression progress and thus they will be overwritten.
- **quality**: in a range from 0 to 100, the quality of the resulting image. **Note** that 0 means _optimization_ (see below). Default: 0.
- **exif_copy**: set it to _true_ to copy EXIF tag info after compression. Default: false.
- **dct_method**: one of the turbojpeg DCT flags. Default: TJFLAG_FASTDCT.
- **scale_factor**: the image scaling factor, expressed as double precision number. Default: 1.0.

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
	double scale_factor;
} cs_png_pars;
```
Those are the zopflipng compression parameters, except for the last one.
- **iterations**: number of iterations (more means more compression). Default: 10.
- **iteration_large**: number of iterations for large files. Default: 5.
- **block_split_strategy**: filter strategy. Default: 4;
- **lossy_8**: convert 16-bit per channel image to 8-bit per channel. Default: true.
- **transparent**: remove colors behind alpha channel 0. Default: true.
- **auto_filter_strategy**: legacy.
- **scale_factor**: the image scaling factor, expressed as double precision number. Note that PNG cannot be upscaled. Default: 1.0.


## Compilation and Installation
Libcaesium uses cmake to build and install the library. Before compiling, be sure to have all the requisites.
Libcaesium requires [mozjpeg](https://github.com/mozilla/mozjpeg) and [zopfli](https://github.com/google/zopfli) installed as shared/static libraries.
Please refer to their own documentation for detailed instructions.
You can also enable the verbose output, which will print on stderr if anything goes wrong, by using the `-DVERBOSE=1` flag during compilation.

### OS X/Linux
##### Requirements
Be sure you have the build tools
###### Linux
`$ sudo apt-get install libtool autoconf git nasm pkg-config cmake libpng-dev`

###### OSX
`$ brew install nasm cmake`

Get the code with
`$ git clone https://github.com/Lymphatus/libcaesium.git`

If you don't have `mozjpeg` and `zopfli` you should run
```bash
$ cd libcaesium
$ ./install.sh
```
which will install the requirements.

##### Compile
Provided you have all the requirements, building and installing from git is as simple as typing
```bash
$ mkdir build
$ cd build
$ cmake ..
$ make
$ sudo make install
```
This will compile the Caesium library, the required header and a small demo application named _caesiumd_.

### Windows
Compiling on Windows is somehow tricky. You can achieve it with MinGW (tested) or Cygwin (not tested), but it's better to stick with the binaries provided.

## Compression vs Optimization
JPEG is a lossy format: that means you will always lose some information after each compression. So, compressing a file with
100 quality for 10 times will result in a always different image, even though you can't really see the difference.
Libcaesium also supports optimization, by setting the _quality_ to 0. This performs a lossless process, resulting in the same image,
but with a smaller size (10-15% usually).  
PNG is lossless, so libcaesium will always perform optimization rather than compression.

## Resizing
Resizing is partially supported. It is handy but it's almost completely out of the scope of this library.
If you really feel the need to do it within libcaesium you can do so, but I advise you should opt for a different toolset for the best results.
