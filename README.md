# libcaesium [![Rust](https://github.com/Lymphatus/libcaesium/actions/workflows/rust.yml/badge.svg)](https://github.com/Lymphatus/libcaesium/actions/workflows/rust.yml)

Libcaesium is a simple library performing JPEG, PNG, WebP and GIF (experimental) compression/optimization written in Rust, with a C interface.\
**IMPORTANT**: starting from v0.6.0 the library is written in Rust and no longer in C. There's a C interface, but it's not backward compatible with the <0.6.0.

## Usage in Rust
Libcaesium exposes two functions, auto-detecting the input file type
### Based on quality values
```Rust
pub fn compress(
    input_path: String,
    output_path: String,
    parameters: &CSParameters
) -> Result<(), Box<dyn Error>>
```
#### Parameters
- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `parameters` - options struct, containing compression parameters (see below)

### Based on output size
```Rust
pub fn compress_to_size(
    input_path: String,
    output_path: String,
    parameters: &CSParameters,
    max_output_size: usize,
) -> Result<(), Box<dyn Error>>
```
#### Parameters
- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `parameters` - options struct, containing compression parameters (see below)
- `max_output_size` - the maximum output size, in bytes

This function will attempt to compress the given file *below* the desired size. It will never exceed it. The function 
will start looping until the best size under the desired is achieved. The function has a 2% tolerance for the output size.
All quality value set to the parameters will be ignored and overwritten during the compression.

NOTE: The output folder where the file is compressed **must** exist.
### Compression options
Libcaesium supports a few compression parameters for each file it supports.
They are defined into a top level struct containing each supported file parameters, as follows:
```Rust
pub struct CSParameters {
    pub jpeg: jpeg::Parameters,
    pub png: png::Parameters,
    pub gif: gif::Parameters,
    pub webp: webp::Parameters,
    pub tiff: tiff::Parameters,
    pub keep_metadata: bool,
    pub optimize: bool,
    pub width: u32,
    pub height: u32,
}
```
Each file type has its own options, but the last two are generic:
- `keep_metadata`: will keep metadata information for any supported type. JPEG and PNG supported. Default `false`.
- `optimize`: forces optimization, when available. With this option enabled the compression will be lossless. JPEG, PNG and WebP supported. Default `false`.
- `width`: Resizes the image to the given width. If this value is `0` and the height value is also `0`, no resizing will be done. If this is `0` and height is `> 0`, the image will be scaled based on height keeping the aspect ratio. Default `0`.
- `height`: Resizes the image to the given height. If this value is `0` and the width value is also `0`, no resizing will be done. If this is `0` and width is `> 0`, the image will be scaled based on width keeping the aspect ratio. Default `0`.

#### jpeg
```Rust
pub struct Parameters {
    pub quality: u32,
    pub chroma_subsampling: jpeg::ChromaSubsampling
}
```
- `quality`: in a range from 1 to 100, the quality of the resulting image. Default `80`.
- `chroma_subsampling`: [chroma subsampling](https://en.wikipedia.org/wiki/Chroma_subsampling) to apply during compression. Default `Auto`.

#### png
```Rust
pub struct Parameters {
    pub quality: u32,
    pub force_zopfli: bool,
    pub optimization_level: u32
}
```
- `quality`: in a range from 0 to 100, the quality of the resulting image. Default `80`.
- `force_zopfli`: if `optimization` is `true` and this option is also `true`, will use zopfli algorithm for compression, resulting in a smaller image, but it may take minutes to finish the process. Default `false`.
- `optimization_level`: if `optimization` is `true` will set the level of oxipng optimization, from 1 to 6. Default `3`.

#### gif
GIF support is experimental, has many know issues and does not support optimization. Expect bugs (especially on Windows).
```Rust
pub struct Parameters {
    pub quality: u32,
}
```
- `quality`: in a range from 0 to 100, the quality of the resulting image. If the optimization flag is `true`, the level is set to `100`. Default: `80`.

#### webp
WebP's compression is tricky. The format is already well optimized and using the `optimize` flag will probably result in a bigger image.
```Rust
pub struct Parameters {
    pub quality: u32,
}
```
- `quality`: in a range from 0 to 100, the quality of the resulting image. If the optimization flag is `true`, this option will be ignored. Default: `60`.

#### tiff
Supported TIFF compression is only lossless. The supported algorithms are: Lzw, Deflate, Packbits, Uncompressed.
```Rust
pub struct Parameters {
    pub algorithm: TiffCompression,
    pub deflate_level: DeflateLevel,
}
```
- `deflate_level`: can be one of `Fast`, `Balanced`, `Best`.

_________________

## Usage in C
Libcaesium exposes two C functions, auto-detecting the input file type:
### Based on quality values
```Rust
pub unsafe extern "C" fn c_compress(
    input_path: *const c_char,
    output_path: *const c_char,
    params: CCSParameters
) -> CCSResult
```
#### Parameters
- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `parameters` - options struct, containing compression parameters (see below)
#### Return
A `CCSResult` struct
```Rust
#[repr(C)]
pub struct CCSResult {
    pub success: bool,
    pub error_message: *const c_char,
}
```
If `success` is `true` the compression process ended successfully and `error_message` will be empty.  
On failure, the `error_message` will be filled with a string containing a brief explanation of the error.

### Based on output size
```Rust
pub unsafe extern "C" fn c_compress_to_size(
    input_path: *const c_char,
    output_path: *const c_char,
    params: CCSParameters,
    max_output_size: usize,
) -> CCSResult
```
#### Parameters
- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `parameters` - options struct, containing compression parameters (see below)
- `max_output_size` - the maximum output size, in bytes
#### Return
A `CCSResult` struct
```Rust
#[repr(C)]
pub struct CCSResult {
    pub success: bool,
    pub error_message: *const c_char,
}
```
If `success` is `true` the compression process ended successfully and `error_message` will be empty.  
On failure, the `error_message` will be filled with a string containing a brief explanation of the error.

### Compression options
The C options struct is slightly different from the Rust one:
```Rust
#[repr(C)]
pub struct CCSParameters {
    pub keep_metadata: bool,
    pub jpeg_quality: u32,
    pub jpeg_chroma_subsampling: u32,
    pub png_quality: u32,
    pub png_optimization_level: u32,
    pub png_force_zopfli: bool,
    pub gif_quality: u32,
    pub webp_quality: u32,
    pub tiff_compression: u32,
    pub tiff_deflate_level: u32,
    pub optimize: bool,
    pub width: u32,
    pub height: u32,
}
```
The option description is the same as the Rust counterpart.
Valid values for `jpeg_chroma_subsampling` are [444, 422, 420, 411]. Any other value will be ignored and will be used the default option.
Valid values for `tiff_compression` are [0 (Uncompressed), 1 (Lzw), 2 (Deflate), 3 (Packbits)]. Any other value will be ignored and `0` will be used.
Valid values for `tiff_deflate_level` are [3 (Fast), 6 (Balanced), 9 (Best)]. Any other value will be ignored and `Best` will be used.

## Download
Binaries not available. Please refer to the compilation section below.

## Compilation and Installation
Compilation is available for all supported platforms: Windows, macOS and Linux.

```
cargo build --release
```
Note: if you don't use the `--release` flag, the PNG optimizations can take a very long time to complete, especially using the zopfli algorithm.

The result will be a dynamic library usable by external applications through its C interface.

## Compression vs Optimization
JPEG is a lossy format: that means you will always lose some information after each compression. So, compressing a file with
100 quality for 10 times will result in an always different image, even though you can't really see the difference.
Libcaesium also supports optimization, by setting the _quality_ to 0. This performs a lossless process, resulting in the same image,
but with a smaller size (10-12% usually).  
GIF optimization is possible, but currently not supported.
WebP's optimization is also possible, but it will probably result in a bigger output file as it's well suited to losslessly convert from PNG or JPEG.
