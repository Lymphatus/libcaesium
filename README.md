# libcaesium

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
}
```
Each file type has its own options, but the last two are generic:
- `keep_metadata`: will keep metadata information for any supported type. JPEG and PNG supported. Default `false`.
- `optimize`: forces optimization, when available. With this option enabled the compression will be lossless. JPEG, PNG and WebP supported. Default `false`.

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
    pub oxipng: oxipng::Options,
    pub level: u32,
    pub force_zopfli: bool
}
```
- `oxipng`: oxipng options. Should be left as default unless you want to do something advanced. Refer to [oxipng](https://github.com/shssoichiro/oxipng) for documentation.
- `level`: level of optimization, from 0 to 6. Increasing the level will result in a smaller file, at the cost of computation time. If the optimization flag is `true`, the level is set to `6`. Default: `3`.
- `force_zopfli`: if `optimization` is `true` and this option is also `true`, will use zopfli algorithm for compression, resulting in a smaller image but it may take minutes to finish the process. Default `false`.

#### gif
GIF support is experimental, has many know issues and does not support optimization. Expect bugs (especially on Windows).
```Rust
pub struct Parameters {
    pub quality: u32,
}
```
- `quality`: in a range from 0 to 100, the quality of the resulting image. If the optimization flag is `true`, the level is set to `100`. Default: `80`.

#### webp
WebP compression is tricky. The format is already well optimized and using the `optimize` flag will probably result in a bigger image.
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
    params: C_CSParameters
) -> bool
```
#### Parameters
- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `parameters` - options struct, containing compression parameters (see below)
- 
#### Return
`true` if all goes well, `false` otherwise.

### Compression options
The C options struct is slightly different from the Rust one:
```Rust
#[repr(C)]
pub struct C_CSParameters {
    pub keep_metadata: bool,
    pub jpeg_quality: u32,
    pub png_level: u32,
    pub png_force_zopfli: bool,
    pub gif_quality: u32,
    pub webp_quality: u32,
    pub optimize: bool,
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
WebP optimization is also possible, but it will probably result in a bigger output file as it's well suited to losslessly convert from PNG or JPEG.

## Resizing
Resizing is no longer supported since 0.6.0.