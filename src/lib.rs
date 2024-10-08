extern crate alloc;

use std::fs;
use std::fs::File;
use std::io::Write;

use error::CaesiumError;
use crate::parameters::{CSParameters, TiffDeflateLevel};
use crate::parameters::TiffCompression::{Deflate, Lzw, Packbits};
use crate::utils::{get_filetype_from_memory, get_filetype_from_path};

pub mod error;
#[cfg(feature = "gif")]
mod gif;
mod interface;
#[cfg(feature = "jpg")]
mod jpeg;
#[cfg(feature = "png")]
mod png;
mod resize;
#[cfg(feature = "tiff")]
mod tiff;
mod utils;
#[cfg(feature = "webp")]
mod webp;
mod convert;
pub mod parameters;

pub fn compress(
    input_path: String,
    output_path: String,
    parameters: &CSParameters,
) -> error::Result<()> {
    validate_parameters(parameters)?;
    let file_type = get_filetype_from_path(&input_path);

    match file_type {
        #[cfg(feature = "jpg")]
        SupportedFileTypes::Jpeg => {
            jpeg::compress(input_path, output_path, parameters)?;
        }
        #[cfg(feature = "png")]
        SupportedFileTypes::Png => {
            png::compress(input_path, output_path, parameters)?;
        }
        #[cfg(feature = "webp")]
        SupportedFileTypes::WebP => {
            webp::compress(input_path, output_path, parameters)?;
        }
        #[cfg(feature = "gif")]
        SupportedFileTypes::Gif => {
            gif::compress(input_path, output_path, parameters)?;
        }
        #[cfg(feature = "tiff")]
        SupportedFileTypes::Tiff => {
            tiff::compress(input_path, output_path, parameters)?;
        }
        _ => {
            return Err(CaesiumError {
                message: "Unknown file type or file not found".into(),
                code: 10000,
            });
        }
    }

    Ok(())
}

pub fn compress_in_memory(
    in_file: Vec<u8>,
    parameters: &CSParameters,
) -> error::Result<Vec<u8>> {
    let file_type = get_filetype_from_memory(in_file.as_slice());
    let compressed_file = match file_type {
        #[cfg(feature = "jpg")]
        SupportedFileTypes::Jpeg => jpeg::compress_in_memory(in_file, parameters)?,
        #[cfg(feature = "png")]
        SupportedFileTypes::Png => png::compress_in_memory(in_file, parameters)?,
        #[cfg(feature = "webp")]
        SupportedFileTypes::WebP => webp::compress_in_memory(in_file, parameters)?,
        #[cfg(feature = "tiff")]
        SupportedFileTypes::Tiff => tiff::compress_in_memory(in_file, parameters)?,
        _ => {
            return Err(CaesiumError {
                message: "Format not supported for compression in memory".into(),
                code: 10200,
            });
        }
    };

    Ok(compressed_file)
}

pub fn compress_to_size_in_memory(
    in_file: Vec<u8>,
    parameters: &mut CSParameters,
    max_output_size: usize,
    return_smallest: bool,
) -> error::Result<Vec<u8>> {
    let file_type = get_filetype_from_memory(&in_file);

    let tolerance_percentage = 2;
    let tolerance = max_output_size * tolerance_percentage / 100;
    let mut quality = 80;
    let mut last_less = 1;
    let mut last_high = 101;
    let max_tries: u32 = 10;
    let mut tries: u32 = 0;

    let compressed_file = match file_type {
        #[cfg(feature = "tiff")]
        SupportedFileTypes::Tiff => {
            let algorithms = [
                Lzw,
                Packbits
            ];
            parameters.tiff.deflate_level = TiffDeflateLevel::Best;
            parameters.tiff.algorithm = Deflate;
            let mut smallest_result = tiff::compress_in_memory(in_file.clone(), parameters)?; //TODO clone
            for tc in algorithms {
                parameters.tiff.algorithm = tc;
                let result = tiff::compress_in_memory(in_file.clone(), parameters)?; //TODO clone
                if result.len() < smallest_result.len() {
                    smallest_result = result;
                }
            }
            return if return_smallest {
                Ok(smallest_result)
            } else {
                Err(CaesiumError {
                    message: "Cannot compress to desired quality".into(),
                    code: 10202,
                })
            };
        }
        _ => loop {
            if tries >= max_tries {
                return Err(CaesiumError {
                    message: "Max tries reached".into(),
                    code: 10201,
                });
            }

            let compressed_file = match file_type {
                #[cfg(feature = "jpg")]
                SupportedFileTypes::Jpeg => {
                    parameters.jpeg.quality = quality;
                    jpeg::compress_in_memory(in_file.clone(), parameters)? //TODO clone
                }
                #[cfg(feature = "png")]
                SupportedFileTypes::Png => {
                    parameters.png.quality = quality;
                    png::compress_in_memory(in_file.clone(), parameters)? //TODO clone
                }
                #[cfg(feature = "webp")]
                SupportedFileTypes::WebP => {
                    parameters.webp.quality = quality;
                    webp::compress_in_memory(in_file.clone(), parameters)? //TODO clone
                }
                _ => {
                    return Err(CaesiumError {
                        message: "Format not supported for compression to size".into(),
                        code: 10200,
                    });
                }
            };

            let compressed_file_size = compressed_file.len();

            if compressed_file_size <= max_output_size
                && max_output_size - compressed_file_size < tolerance
            {
                break compressed_file;
            }

            if compressed_file_size <= max_output_size {
                last_less = quality;
            } else {
                last_high = quality;
            }
            let last_quality = quality;
            quality = ((last_high + last_less) / 2).clamp(1, 100);
            if last_quality == quality {
                if quality == 1 && last_high == 1 {
                    return if return_smallest {
                        Ok(compressed_file)
                    } else {
                        Err(CaesiumError {
                            message: "Cannot compress to desired quality".into(),
                            code: 10202,
                        })
                    };
                }

                break compressed_file;
            }

            tries += 1;
        },
    };

    Ok(compressed_file)
}

pub fn compress_to_size(
    input_path: String,
    output_path: String,
    parameters: &mut CSParameters,
    max_output_size: usize,
    return_smallest: bool,
) -> error::Result<()> {
    let in_file = fs::read(input_path.clone()).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 10201,
    })?;
    let original_size = in_file.len();
    if original_size <= max_output_size {
        fs::copy(input_path, output_path).map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 10202,
        })?;
        return Ok(());
    }
    let compressed_file =
        compress_to_size_in_memory(in_file, parameters, max_output_size, return_smallest)?;
    let mut out_file = File::create(output_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 10203,
    })?;
    out_file
        .write_all(&compressed_file)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 10204,
        })?;

    Ok(())
}

pub fn convert(input_path: String, output_path: String, parameters: &CSParameters, format: SupportedFileTypes) -> error::Result<()> {

    let file_type = get_filetype_from_path(&input_path);

    if file_type == format {
        return Err(CaesiumError {
            message: "Cannot convert to the same format".into(),
            code: 10406,
        });
    }

    let in_file = fs::read(input_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 10410,
    })?;
    let output_buffer = convert_in_memory(in_file, parameters, format).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 10411,
    })?;

    let mut out_file = File::create(output_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 10412,
    })?;

    out_file.write_all(&output_buffer).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 10413,
    })?;

    Ok(())
}
pub fn convert_in_memory(in_file: Vec<u8>, parameters: &CSParameters, format: SupportedFileTypes) -> Result<Vec<u8>, CaesiumError> {
    convert::convert_in_memory(in_file, format, parameters)
}

fn validate_parameters(parameters: &CSParameters) -> error::Result<()> {
    if parameters.jpeg.quality > 100 {
        return Err(CaesiumError {
            message: "Invalid JPEG quality value".into(),
            code: 10001,
        });
    }

    if parameters.png.quality > 100 {
        return Err(CaesiumError {
            message: "Invalid PNG quality value".into(),
            code: 10002,
        });
    }

    if parameters.png.optimization_level > 6 {
        return Err(CaesiumError {
            message: "Invalid PNG optimization level".into(),
            code: 10006,
        });
    }

    if parameters.gif.quality > 100 {
        return Err(CaesiumError {
            message: "Invalid GIF quality value".into(),
            code: 10003,
        });
    }

    if parameters.webp.quality > 100 {
        return Err(CaesiumError {
            message: "Invalid WebP quality value".into(),
            code: 10004,
        });
    }

    if parameters.optimize && parameters.output_size > 0 {
        return Err(CaesiumError {
            message: "Cannot compress to desired size with lossless optimization".into(),
            code: 10005,
        });
    }

    Ok(())
}

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