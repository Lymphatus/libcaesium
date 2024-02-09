use std::fs;
use std::fs::File;
use std::io::Write;
use libheif_rs::{ColorSpace, HeifContext, LibHeif, CompressionFormat, EncoderQuality};
use crate::CSParameters;
use crate::utils::CaesiumError;

pub fn compress(
    input_path: String,
    output_path: String,
    parameters: &CSParameters,
    format: CompressionFormat
) -> Result<(), CaesiumError> {
    let in_file = fs::read(input_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20500,
    })?;

    if parameters.width > 0 || parameters.height > 0 {
        //TODO Resize
    }

    let compressed_image = compress_to_memory(in_file, parameters, format)?;
    let mut output_file_buffer = File::create(output_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20502,
    })?;
    output_file_buffer
        .write_all(compressed_image.as_slice())
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20503,
        })?;

    Ok(())
}

pub fn compress_to_memory(
    in_file: Vec<u8>,
    parameters: &CSParameters,
    format: CompressionFormat
) -> Result<Vec<u8>, CaesiumError> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_bytes(in_file.as_slice())
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20504,
        })?;
    let handle = ctx.primary_image_handle()
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20505,
        })?;

    // Get Exif
    // let mut meta_ids: Vec<ItemId> = vec![0; 1];
    // let count = handle.metadata_block_ids(&mut meta_ids, b"Exif");
    // assert_eq!(count, 1);
    // let exif: Vec<u8> = handle.metadata(meta_ids[0])?;

    // Decode the image
    let mut image = lib_heif.decode(&handle, ColorSpace::Undefined, None)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20506,
        })?;


    if parameters.width > 0 || parameters.height > 0 {
        let resized_image = image.scale(parameters.width, parameters.height, None)
            .map_err(|e| CaesiumError {
                message: e.to_string(),
                code: 20511,
            })?;
        image = resized_image;
    }

    let mut context = HeifContext::new()
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20506,
        })?;
    let mut encoder = lib_heif.encoder_for_format(format)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20507,
        })?;
    let encoder_quality = if parameters.optimize {
        EncoderQuality::LossLess
    } else {
        let quality = match format {
            CompressionFormat::Av1 => parameters.avif.quality as u8,
            CompressionFormat::Hevc => parameters.heic.quality as u8,
            _ => 80
        };
        EncoderQuality::Lossy(quality)
    };
    encoder.set_quality(encoder_quality)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20508,
        })?;
    context.encode_image(&image, &mut encoder, None)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20509,
        })?;

    let compressed_image = context.write_to_bytes()
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20510,
        })?;
    Ok(compressed_image)
}