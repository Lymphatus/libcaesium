# libcaesium

[![Test](https://github.com/Lymphatus/libcaesium/actions/workflows/test.yml/badge.svg)](https://github.com/Lymphatus/libcaesium/actions/workflows/test.yml)
[![Clippy](https://github.com/Lymphatus/caesium-clt/actions/workflows/clippy.yml/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions/workflows/clippy.yml)
[![Code formatting](https://github.com/Lymphatus/caesium-clt/actions/workflows/fmt.yml/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions/workflows/fmt.yml)

Libcaesium is a simple library performing JPEG, PNG, WebP, TIFF and GIF (partial) compression/optimization written in
Rust, with a C interface.

## Usage example

Libcaesium exposes several functions for compressing and converting images, both from files and in-memory buffers.

### Compress an image file

```rust
use caesium::parameters::CSParameters;
use caesium::compress;

let mut parameters = CSParameters::new();
parameters.keep_metadata = true;
parameters.jpeg.quality = 60;

let input_file_path = "input.jpg".to_string();
let output_file_path = "output.jpg".to_string();

let result = compress(input_file_path, output_file_path, &parameters);
assert!(result.is_ok());
```

### Compress an image in memory

```rust
use caesium::parameters::CSParameters;
use caesium::compress_in_memory;
use std::fs;

let parameters = CSParameters::new();
let image_bytes = fs::read("input.png").unwrap();

let compressed_bytes = compress_in_memory(image_bytes, &parameters).unwrap();
// You can now write `compressed_bytes` to a file or use it as needed
```

### Compress an image to a target size

```rust
use caesium::parameters::CSParameters;
use caesium::compress_to_size;

let mut parameters = CSParameters::new();
let input_file_path = "input.webp".to_string();
let output_file_path = "output.webp".to_string();
let max_output_size = 100_000; // 100 KB

let result = compress_to_size(input_file_path, output_file_path, &mut parameters, max_output_size, true);
assert!(result.is_ok());
```

### Convert an image to another format

```rust
use caesium::{parameters::CSParameters, convert, SupportedFileTypes};

let parameters = CSParameters::new();
let input_file_path = "input.png".to_string();
let output_file_path = "output.jpg".to_string();

let result = convert(input_file_path, output_file_path, &parameters, SupportedFileTypes::Jpeg);
assert!(result.is_ok());
```

### Convert an image in memory

```rust
use caesium::{parameters::CSParameters, convert_in_memory, SupportedFileTypes};
use std::fs;

let parameters = CSParameters::new();
let image_bytes = fs::read("input.tiff").unwrap();

let converted_bytes = convert_in_memory(image_bytes, &parameters, SupportedFileTypes::Png).unwrap();
// Use `converted_bytes` as needed
```

You can find more real-world usage in the [examples](examples) folder.  
To run an example, use:

```bash
cargo run --example <example_name>
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

*You can find the C header file in the include folder in the project root directory.*

Libcaesium exposes C functions, auto-detecting the input file type:

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
    pub code: u32,
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
    return_smallest: bool,
) -> CCSResult
```

#### Parameters

- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `parameters` - options struct, containing compression parameters (see below)
- `max_output_size` - the maximum output size, in bytes
- `return_smallest` - whether to return the smallest

#### Return

A `CCSResult` struct

```Rust
#[repr(C)]
pub struct CCSResult {
    pub success: bool,
    pub code: u32,
    pub error_message: *const c_char,
}
```

If `success` is `true` the compression process ended successfully and `error_message` will be empty.  
On failure, the `error_message` will be filled with a string containing a brief explanation of the error.

### Based on convert output

```Rust
pub unsafe extern "C" fn c_convert(
    input_path: *const c_char,
    output_path: *const c_char,
    format: SupportedFileTypes,
    params: CCSParameters,
) -> CCSResult
```

#### Parameters

- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `format` - target image format (see below)
- `parameters` - options struct, containing compression parameters (see below)

#### Return

A `CCSResult` struct

```Rust
#[repr(C)]
pub struct CCSResult {
    pub success: bool,
    pub code: u32,
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
    pub jpeg_optimize: bool,
    pub png_quality: u32,
    pub png_optimization_level: u32,
    pub png_force_zopfli: bool,
    pub png_optimize: bool,
    pub gif_quality: u32,
    pub webp_quality: u32,
    pub webp_lossless: bool,
    pub tiff_compression: u32,
    pub tiff_deflate_level: u32,
    pub optimize: bool,
    pub width: u32,
    pub height: u32,
}
```

The option description is the same as the Rust counterpart.  
Valid values for `jpeg_chroma_subsampling` are `[444, 422, 420, 411]`. Any other value will be ignored and will be used
as
the default option.  
Valid values for `tiff_compression` are `[0 (Uncompressed), 1 (Lzw), 2 (Deflate), 3 (Packbits)]`. Any other value will
be ignored and `0` will be used.  
Valid values for `tiff_deflate_level` are `[1 (Fast), 6 (Balanced), 9 (Best)]`. Any other value will be ignored and
`Best`
will be used.

### Supported file types

```rust
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SupportedFileTypes {
    Jpeg,
    Png,
    Gif,
    WebP,
    Tiff,
    Unkn,
}
```

## Compression vs Optimization

JPEG is a lossy format: that means you will always lose some information after each compression. So, compressing a file
with quality 100 for 10 times will result in an always different image, even though you can't really see the difference.
Libcaesium also supports optimization. This performs a lossless process, resulting in the
same exact image, but with a smaller size (10–12% usually).  
GIF optimization is possible but currently not supported.
WebP's optimization is also possible, but it will probably result in a bigger output file as it's well suited to
losslessly convert from PNG or JPEG.
