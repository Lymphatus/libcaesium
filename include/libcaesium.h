#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum SupportedFileTypes {
  Jpeg,
  Png,
  Gif,
  WebP,
  Tiff,
  Unkn,
} SupportedFileTypes;

typedef struct CCSResult {
  bool success;
  uint32_t code;
  const char *error_message;
} CCSResult;

typedef struct CCSParameters {
  bool keep_metadata;
  uint32_t jpeg_quality;
  uint32_t jpeg_chroma_subsampling;
  bool jpeg_progressive;
  bool jpeg_optimize;
  bool jpeg_preserve_icc;
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

typedef struct CByteArray {
  uint8_t *data;
  uintptr_t length;
} CByteArray;

struct CCSResult c_compress(const char *input_path,
                            const char *output_path,
                            struct CCSParameters params);

struct CCSResult c_compress_in_memory(const uint8_t *input_data,
                                      uintptr_t input_length,
                                      struct CCSParameters params,
                                      struct CByteArray *output);

struct CCSResult c_compress_to_size(const char *input_path,
                                    const char *output_path,
                                    struct CCSParameters params,
                                    uintptr_t max_output_size,
                                    bool return_smallest);

struct CCSResult c_convert(const char *input_path,
                           const char *output_path,
                           enum SupportedFileTypes format,
                           struct CCSParameters params);

void c_free_byte_array(struct CByteArray byte_array);

void c_free_string(char *ptr);
