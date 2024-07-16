use std::io::Cursor;

use bytes::Bytes;
use image::ImageFormat;
use image::io::Reader as ImageReader;
use img_parts::{DynImage, ImageEXIF, ImageICC};

use crate::error::CaesiumError;
use crate::{compress_in_memory, CSParameters, SupportedFileTypes};
use crate::utils::get_filetype_from_memory;

pub fn convert_in_memory(in_file: Vec<u8>, format: SupportedFileTypes, parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
    let output_format = map_image_format(format)?;

    let file_type = get_filetype_from_memory(&in_file);

    if file_type == format {
        return Err(CaesiumError {
            message: "Cannot convert to the same format".into(),
            code: 10407,
        });
    }

    let i = in_file.as_slice();
    let original_image = ImageReader::new(Cursor::new(i)).with_guessed_format()
        .map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 10402,
    })?.decode()
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 10403,
        })?;

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
        let (iccp, exif) = DynImage::from_bytes(Bytes::from(in_file))
            .map_err(|e| CaesiumError {
                message: e.to_string(),
                code: 10401,
            })?
            .map_or((None, None), |dimg| (dimg.icc_profile(), dimg.exif()));

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

        // match output_format {
        //     ImageFormat::Png => {
        //         let mut decoder = WebPDecoder::new(Cursor::new(compressed_converted_image.as_slice()))
        //             .map_err(|e| CaesiumError {
        //             message: e.to_string(),
        //             code: 10405,
        //         })?;
        //         let bytes_per_pixel = if decoder.has_alpha() { 4 } else { 3 };
        //         let (width, height) = decoder.dimensions();
        //         let mut data = vec![0; width as usize * height as usize * bytes_per_pixel];
        //         decoder.read_image(&mut data)
        //             .map_err(|e| CaesiumError {
        //             message: e.to_string(),
        //             code: 10406,
        //         })?;
        //         let mut output_image_with_metadata: Vec<u8> = Vec::new();
        //         let mut encoder = WebPEncoder::new(&mut output_image_with_metadata);
        //         if iccp.is_some() {
        //             encoder.set_icc_profile(iccp.unwrap_or(Bytes::new()).to_vec());
        //         }
        //         if exif.is_some() {
        //             encoder.set_exif_metadata(exif.unwrap_or(Bytes::new()).to_vec());
        //         }
        //
        //         let color_type = match bytes_per_pixel {
        //             4 => Rgba8,
        //             _ => Rgb8
        //         };
        //         encoder.encode(data.as_slice(), width, height, color_type)
        //             .map_err(|e| CaesiumError {
        //             message: e.to_string(),
        //             code: 10407,
        //         })?;
        //
        //         Ok(output_image_with_metadata)
        //     }
        //     _ => {
        //
        //     }
        // }
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