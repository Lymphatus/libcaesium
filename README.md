# libcaesium [![Rust](https://github.com/Lymphatus/libcaesium/actions/workflows/rust.yml/badge.svg)](https://github.com/Lymphatus/libcaesium/actions/workflows/rust.yml)

Libcaesium is a simple library performing JPEG, PNG, WebP and GIF (experimental) compression/optimization written in Rust, with a C interface.\
**IMPORTANT**: starting from v0.6.0 the library is written in Rust and no longer in C. There's a C interface, but it's not backward compatible with the <0.6.0.

## Usage in Rust
Libcaesium exposes one single function, auto-detecting the input file type:
```Rust
pub fn compress(
    input_path: String,
    output_path: String,
    parameters: CSParameters
) -> Result<(), Box<dyn Error>>
```
#### Parameters
- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `parameters` - options struct, containing compression parameters (see below)

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
}
```
- `quality`: in a range from 1 to 100, the quality of the resulting image. Default `80`.

#### png
```Rust
pub struct Parameters {
    pub quality: u32,
    pub force_zopfli: bool
}
```
- `quality`: in a range from 0 to 100, the quality of the resulting image. Default `80`.
- `force_zopfli`: if `optimization` is `true` and this option is also `true`, will use zopfli algorithm for compression, resulting in a smaller image, but it may take minutes to finish the process. Default `false`.

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

## Usage in C
Libcaesium exposes one single C function, auto-detecting the input file type:
```Rust
pub extern fn c_compress(
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

### Compression options
The C options struct is slightly different from the Rust one:
```Rust
#[repr(C)]
pub struct CCSParameters {
    pub keep_metadata: bool,
    pub jpeg_quality: u32,
    pub png_quality: u32,
    pub png_force_zopfli: bool,
    pub gif_quality: u32,
    pub webp_quality: u32,
    pub optimize: bool,
    pub width: u32,
    pub height: u32,
}
```
The option description is the same as the Rust counterpart.

## Download
Binaries not available. Please refer to the compilation section below.

## Compilation and Installation
Compilation is available for all supported platforms: Windows, MacOS and Linux.

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
PNG is lossless, so libcaesium will always perform optimization rather than compression.
GIF optimization is possible, but currently not supported.
WebP's optimization is also possible, but it will probably result in a bigger output file as it's well suited to losslessly convert from PNG or JPEG.s
