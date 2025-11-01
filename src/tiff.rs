use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::panic;

use image::ImageFormat::Tiff;
use tiff::encoder::colortype::{RGB8, RGBA8};
use tiff::encoder::compression::{Deflate, DeflateLevel, Lzw, Packbits, Uncompressed};
use tiff::encoder::TiffEncoder;

use crate::error::CaesiumError;
use crate::parameters::TiffCompression;
use crate::resize::resize_image;
use crate::{CSParameters, TiffDeflateLevel};

pub fn compress(input_path: String, output_path: String, parameters: &CSParameters) -> Result<(), CaesiumError> {
    let mut input_file = File::open(input_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20500,
    })?;

    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20501,
    })?;

    let compressed_image = compress_in_memory(&input_data, parameters)?;

    let mut output_file = File::create(output_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20502,
    })?;

    output_file.write_all(&compressed_image).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20503,
    })?;
    Ok(())
}

pub fn compress_in_memory(in_file: &Vec<u8>, parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
    let decoding_result = match panic::catch_unwind(|| image::load_from_memory_with_format(in_file.as_slice(), Tiff)) {
        Ok(i) => i,
        Err(_) => {
            return Err(CaesiumError {
                message: "Failed to decode TIFF image".to_string(),
                code: 20504,
            });
        }
    };
    let mut image = match decoding_result {
        Ok(i) => i,
        Err(e) => {
            return Err(CaesiumError {
                message: e.to_string(),
                code: 20504,
            })
        }
    };

    if parameters.width > 0 || parameters.height > 0 {
        image = resize_image(image, parameters.width, parameters.height);
    }

    let color_type = image.color();
    let output_buff = vec![];
    let mut output_stream = Cursor::new(output_buff);
    let mut encoder = TiffEncoder::new(&mut output_stream).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20505,
    })?;

    macro_rules! write_with_compression {
        ($compression:expr) => {
            match color_type {
                image::ColorType::Rgb8 => encoder.write_image_with_compression::<RGB8, _>(
                    image.width(),
                    image.height(),
                    $compression,
                    image.as_bytes(),
                ),
                image::ColorType::Rgba8 => encoder.write_image_with_compression::<RGBA8, _>(
                    image.width(),
                    image.height(),
                    $compression,
                    image.as_bytes(),
                ),
                _ => {
                    return Err(CaesiumError {
                        message: format!("Unsupported TIFF color type ({color_type:?})"),
                        code: 20506,
                    });
                }
            }
        };
    }

    let compression_result = match parameters.tiff.algorithm {
        TiffCompression::Deflate => {
            write_with_compression!(Deflate::with_level(parse_deflate_level(parameters.tiff.deflate_level)))
        }
        TiffCompression::Lzw => write_with_compression!(Lzw),
        TiffCompression::Packbits => write_with_compression!(Packbits),
        TiffCompression::Uncompressed => write_with_compression!(Uncompressed),
    };

    match compression_result {
        Ok(_) => Ok(output_stream.get_ref().to_vec()),
        Err(e) => Err(CaesiumError {
            message: e.to_string(),
            code: 20507,
        }),
    }
}

fn parse_deflate_level(level: TiffDeflateLevel) -> DeflateLevel {
    match level {
        TiffDeflateLevel::Fast => DeflateLevel::Fast,
        TiffDeflateLevel::Best => DeflateLevel::Best,
        TiffDeflateLevel::Balanced => DeflateLevel::Balanced,
    }
}
