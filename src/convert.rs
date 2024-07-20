use std::io::Cursor;

use bytes::Bytes;
use image::{ColorType, DynamicImage, ImageFormat};
use image::io::Reader as ImageReader;
use img_parts::{DynImage, ImageEXIF, ImageICC};

use crate::{compress_in_memory, CSParameters, SupportedFileTypes};
use crate::error::CaesiumError;
use crate::utils::get_filetype_from_memory;

pub fn convert_in_memory(in_file: Vec<u8>, format: SupportedFileTypes, parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
    let mut iccp = None;
    let mut exif = None;

    let output_format = map_image_format(format)?;
    let original_file_type = get_filetype_from_memory(&in_file);

    if original_file_type == format {
        return Err(CaesiumError {
            message: "Cannot convert to the same format".into(),
            code: 10407,
        });
    }

    let i = in_file.as_slice();
    let mut original_image = ImageReader::new(Cursor::new(i)).with_guessed_format()
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 10402,
        })?.decode()
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 10403,
        })?;

    if parameters.keep_metadata {
        if original_file_type == SupportedFileTypes::Jpeg {
            let orientation = get_jpeg_orientation(in_file.as_slice());
            original_image = match orientation {
                2 => original_image.fliph(),
                3 => original_image.rotate180(),
                4 => original_image.flipv(),
                5 => original_image.rotate90().fliph(),
                6 => original_image.rotate90(),
                7 => original_image.fliph().rotate90(),
                8 => original_image.rotate270(),
                _ => original_image
            }
        }

        (iccp, exif) = DynImage::from_bytes(Bytes::from(in_file))
            .map_err(|e| CaesiumError {
                message: e.to_string(),
                code: 10401,
            })?
            .map_or((None, None), |dimg| (dimg.icc_profile(), dimg.exif()));
    }

    if format == SupportedFileTypes::Jpeg {
        original_image = match original_image.color() {
            ColorType::Rgba8 => DynamicImage::from(original_image.to_rgb8()),
            ColorType::Rgba16 => DynamicImage::from(original_image.to_rgb16()),
            ColorType::Rgba32F => DynamicImage::from(original_image.to_rgb32f()),
            _ => original_image,
        };
    }

    let mut output_image: Vec<u8> = Vec::new();
    original_image.write_to(&mut Cursor::new(&mut output_image), output_format)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 10404,
        })?;

    let compressed_converted_image = compress_in_memory(output_image, parameters)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 10405,
        })?;

    if parameters.keep_metadata {
        let dyn_image = DynImage::from_bytes(Bytes::from(compressed_converted_image.clone()))
            .map_err(|e| CaesiumError {
                message: e.to_string(),
                code: 10408,
            })?;

        match dyn_image {
            Some(mut dimg) => {
                let mut output_image_with_metadata: Vec<u8> = Vec::new();
                if iccp.is_some() {
                    dimg.set_icc_profile(iccp);
                }
                if exif.is_some() {
                    dimg.set_exif(exif);
                }
                dimg.encoder()
                    .write_to(&mut output_image_with_metadata)
                    .map_err(|e| CaesiumError {
                        message: e.to_string(),
                        code: 10409,
                    })?;

                Ok(output_image_with_metadata)
            }
            None => {
                Ok(compressed_converted_image)
            }
        }
    } else {
        Ok(compressed_converted_image)
    }
}

fn map_image_format(format: SupportedFileTypes) -> Result<ImageFormat, CaesiumError> {
    let image_format = match format {
        SupportedFileTypes::Jpeg => ImageFormat::Jpeg,
        SupportedFileTypes::Png => ImageFormat::Png,
        SupportedFileTypes::Gif => ImageFormat::Gif,
        SupportedFileTypes::WebP => ImageFormat::WebP,
        SupportedFileTypes::Tiff => ImageFormat::Tiff,
        _ => return Err(CaesiumError {
            message: "Output format is unknown".into(),
            code: 10400,
        })
    };

    Ok(image_format)
}

fn get_jpeg_orientation(data: &[u8]) -> u32 {
    let reader = exif::Reader::new();
    let mut cursor = Cursor::new(data);

    let exif_data = match reader.read_from_container(&mut cursor) {
        Ok(v) => v,
        Err(_) => return 1
    };

    let exif_field = match exif_data.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
        Some(value) => value,
        None => return 1,
    };

    exif_field.value.get_uint(0).unwrap_or(1)
}
