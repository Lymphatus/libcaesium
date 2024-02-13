extern crate alloc;

use crate::jpeg::ChromaSubsampling;
use crate::tiff::TiffCompression;
use crate::tiff::TiffCompression::Deflate;
use crate::utils::{get_filetype_from_memory, get_filetype_from_path, SupportedFileTypes};
use ::tiff::encoder::compression::DeflateLevel;
use error::CaesiumError;
use std::fs::File;
use std::io::Write;
use std::{cmp, fs};

mod error;
#[cfg(feature = "gif")]
mod gif;
mod interface;
#[cfg(feature = "jpg")]
pub mod jpeg;
#[cfg(feature = "png")]
mod png;
mod resize;
#[cfg(feature = "tiff")]
mod tiff;
mod utils;
#[cfg(feature = "webp")]
mod webp;

#[derive(Copy, Clone)]
pub struct JpegParameters {
    pub quality: u32,
    pub chroma_subsampling: ChromaSubsampling,
}

#[derive(Copy, Clone)]
pub struct PngParameters {
    pub quality: u32,
    pub force_zopfli: bool,
}

#[derive(Copy, Clone)]
pub struct GifParameters {
    pub quality: u32,
}

#[derive(Copy, Clone)]
pub struct WebPParameters {
    pub quality: u32,
}

#[derive(Copy, Clone)]
pub struct TiffParameters {
    pub algorithm: TiffCompression,
    pub deflate_level: DeflateLevel,
}

#[derive(Copy, Clone)]
pub struct CSParameters {
    pub jpeg: JpegParameters,
    pub png: PngParameters,
    pub gif: GifParameters,
    pub webp: WebPParameters,
    pub tiff: TiffParameters,
    pub keep_metadata: bool,
    pub optimize: bool,
    pub width: u32,
    pub height: u32,
    pub output_size: u32,
}

pub fn initialize_parameters() -> CSParameters {
    let jpeg = JpegParameters {
        quality: 80,
        chroma_subsampling: ChromaSubsampling::Auto,
    };
    let png = PngParameters {
        quality: 80,
        force_zopfli: false,
    };
    let gif = GifParameters { quality: 80 };
    let webp = WebPParameters { quality: 80 };
    let tiff = TiffParameters {
        algorithm: Deflate,
        deflate_level: DeflateLevel::Balanced,
    };

    CSParameters {
        jpeg,
        png,
        gif,
        webp,
        tiff,
        keep_metadata: false,
        optimize: false,
        width: 0,
        height: 0,
        output_size: 0,
    }
}

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
                message: "Unknown file type".into(),
                code: 10000,
            });
        }
    }

    Ok(())
}

pub fn compress_in_memory(
    in_file: Vec<u8>,
    parameters: &mut CSParameters,
) -> error::Result<Vec<u8>> {
    let file_type = get_filetype_from_memory(in_file.as_slice());
    let compressed_file = match file_type {
        #[cfg(feature = "jpg")]
        SupportedFileTypes::Jpeg => jpeg::compress_to_memory(in_file, parameters)?,
        #[cfg(feature = "png")]
        SupportedFileTypes::Png => png::compress_to_memory(in_file, parameters)?,
        #[cfg(feature = "webp")]
        SupportedFileTypes::WebP => webp::compress_to_memory(in_file, parameters)?,
        #[cfg(feature = "tiff")]
        SupportedFileTypes::Tiff => tiff::compress_to_memory(in_file, parameters)?,
        _ => {
            return Err(CaesiumError {
                message: "Format not supported for compression to size".into(),
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

    let compressed_file = loop {
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
                jpeg::compress_to_memory(in_file.clone(), parameters)? //TODO clone
            }
            #[cfg(feature = "png")]
            SupportedFileTypes::Png => {
                parameters.png.quality = quality;
                png::compress_to_memory(in_file.clone(), parameters)? //TODO clone
            }
            #[cfg(feature = "webp")]
            SupportedFileTypes::WebP => {
                parameters.webp.quality = quality;
                webp::compress_to_memory(in_file.clone(), parameters)? //TODO clone
            }
            //TODO Tiff
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
        quality = cmp::max(1, cmp::min(100, (last_high + last_less) / 2));
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

fn validate_parameters(parameters: &CSParameters) -> error::Result<()> {
    if parameters.jpeg.quality == 0 || parameters.jpeg.quality > 100 {
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
