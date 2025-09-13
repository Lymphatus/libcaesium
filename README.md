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

### Compress an image file

```c
struct CCSResult c_compress(
    const char *input_path,
    const char *output_path,
    struct CCSParameters params
);
```

#### Parameters

- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `params` - options struct, containing compression parameters (see below)

#### Return

A `CCSResult` struct

```c
typedef struct CCSResult {
    bool success;
    uint32_t code;
    const char *error_message;
} CCSResult;
```

If `success` is `true` the compression process ended successfully and `error_message` will be empty.  
On failure, the `error_message` will be filled with a string containing a brief explanation of the error.

### Compress an image in memory

```c
struct CCSResult c_compress_in_memory(
    const uint8_t *input_data,
    uintptr_t input_length,
    struct CCSParameters params,
    struct CByteArray *output
);
```

#### Parameters

- `input_data` - pointer to input image data
- `input_length` - length of input data in bytes
- `params` - options struct, containing compression parameters (see below)
- `output` - pointer to a `CByteArray` struct that will be filled with the compressed data

#### Return

A `CCSResult` struct (see above).

After use, free the output buffer with `c_free_byte_array`.

### Compress an image to a target size

```c
struct CCSResult c_compress_to_size(
    const char *input_path,
    const char *output_path,
    struct CCSParameters params,
    uintptr_t max_output_size,
    bool return_smallest
);
```

#### Parameters

- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `params` - options struct, containing compression parameters (see below)
- `max_output_size` - the maximum output size, in bytes
- `return_smallest` - whether to return the smallest

#### Return

A `CCSResult` struct (see above).

### Convert an image to another format

```c
struct CCSResult c_convert(
    const char *input_path,
    const char *output_path,
    enum SupportedFileTypes format,
    struct CCSParameters params
);
```

#### Parameters

- `input_path` - input file path (full filename)
- `output_path` - output file path (full filename)
- `format` - target image format (see below)
- `params` - options struct, containing compression parameters (see below)

#### Return

A `CCSResult` struct (see above).

### Memory management helpers

After using functions that allocate memory (such as `c_compress_in_memory`), you must free the returned buffers:

```c
void c_free_byte_array(struct CByteArray byte_array);
void c_free_string(char *ptr);
```

- `c_free_byte_array` frees the memory allocated for a `CByteArray`'s data.
- `c_free_string` frees a string allocated by the library.

### Compression options

The C options struct is as follows:

```c
typedef struct CCSParameters {
    bool keep_metadata;
    uint32_t jpeg_quality;
    uint32_t jpeg_chroma_subsampling;
    bool jpeg_progressive;
    bool jpeg_optimize;
    uint32_t png_quality;
    uint32_t png_optimization_level;
    bool png_force_zopfli;
    bool png_optimize;
    uint32_t gif_quality;
    uint32_t webp_quality;
    bool webp_lossless;
    uint32_t tiff_compression;
    uint32_t tiff_deflate_level;
    uint32_t width;
    uint32_t height;
} CCSParameters;
```

- `keep_metadata`: preserve image metadata (EXIF, etc.)
- `jpeg_quality`: JPEG quality (0-100)
- `jpeg_chroma_subsampling`: JPEG chroma subsampling (`444`, `422`, `420`, `411`)
- `jpeg_progressive`: enable progressive JPEG
- `jpeg_optimize`: enable JPEG optimization
- `png_quality`: PNG quality (0-100)
- `png_optimization_level`: PNG optimization level
- `png_force_zopfli`: force Zopfli compression for PNG
- `png_optimize`: enable PNG optimization
- `gif_quality`: GIF quality (0-100)
- `webp_quality`: WebP quality (0-100)
- `webp_lossless`: enable WebP lossless mode
- `tiff_compression`: TIFF compression (`0`=Uncompressed, `1`=Lzw, `2`=Deflate, `3`=Packbits)
- `tiff_deflate_level`: TIFF deflate level (`1`=Fast, `6`=Balanced, `9`=Best)
- `width`, `height`: resize output image (set to `0` to keep original size)

### Byte array struct

```c
typedef struct CByteArray {
    uint8_t *data;
    uintptr_t length;
} CByteArray;
```

- `data`: pointer to the buffer
- `length`: length of the buffer in bytes

### Supported file types

```c
typedef enum SupportedFileTypes {
    Jpeg,
    Png,
    Gif,
    WebP,
    Tiff,
    Unkn,
} SupportedFileTypes;
```

## Compression vs Optimization

JPEG is a lossy format: that means you will always lose some information after each compression. So, compressing a file
with quality 100 for 10 times will result in an always different image, even though you can't really see the difference.
Libcaesium also supports optimization. This performs a lossless process, resulting in the
same exact image, but with a smaller size (10–12% usually).  
GIF optimization is possible but currently not supported.
WebP's optimization is also possible, but it will probably result in a bigger output file as it's well suited to
losslessly convert from PNG or JPEG.
