# libcaesium [![Rust](https://github.com/Lymphatus/libcaesium/actions/workflows/rust.yml/badge.svg)](https://github.com/Lymphatus/libcaesium/actions/workflows/rust.yml)

Libcaesium is a simple library performing JPEG, PNG, WebP and GIF (experimental) compression/optimization written in
Rust, with a C interface.

> [!WARNING]
> starting from v0.6.0 the library is written in Rust and no longer in C. There's a C interface, but it's not backward
> compatible with the <0.6.0.

## Usage example

Libcaesium exposes two functions, auto-detecting the input file type

```Rust
use caesium::parameters::CSParameters;
use caesium::compress;

let mut parameters = CSParameters::new();
parameters.keep_metadata = true;
parameters.jpeg.quality = 60;

let success = compress(input, output, &parameters).is_ok();
```

## Compilation

Compilation is available for all supported platforms: Windows, macOS and Linux.

> [!NOTE]
> if you don't use the `--release` flag, the PNG optimizations can take a very long time to complete, especially
> using the zopfli algorithm.
>

```bash
cargo build --release
```

The result will be a dynamic library usable by external applications through its C interface.

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
    pub jpeg_progressive: bool,
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
Valid values for `jpeg_chroma_subsampling` are `[444, 422, 420, 411]`. Any other value will be ignored and will be used
the default option.  
Valid values for `tiff_compression` are `[0 (Uncompressed), 1 (Lzw), 2 (Deflate), 3 (Packbits)]`. Any other value will be
ignored and `0` will be used.  
Valid values for `tiff_deflate_level` are `[1 (Fast), 6 (Balanced), 9 (Best)]`. Any other value will be ignored and `Best`
will be used.

## Compression vs Optimization

JPEG is a lossy format: that means you will always lose some information after each compression. So, compressing a file
with 100 quality for 10 times will result in an always different image, even though you can't really see the difference.
Libcaesium also supports optimization, by setting the _quality_ to 0. This performs a lossless process, resulting in the
same image, but with a smaller size (10-12% usually).  
GIF optimization is possible, but currently not supported.
WebP's optimization is also possible, but it will probably result in a bigger output file as it's well suited to
losslessly convert from PNG or JPEG.
