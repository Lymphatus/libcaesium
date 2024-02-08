use std::fs;
use std::fs::File;
use std::io::Write;
use libheif_rs::{Channel, RgbChroma, ColorSpace, HeifContext, Result, ItemId, LibHeif, CompressionFormat, EncoderQuality};
use crate::CSParameters;
use crate::utils::CaesiumError;

pub fn compress(
    input_path: String,
    output_path: String,
    parameters: &CSParameters,
) -> std::result::Result<(), CaesiumError> {
    let in_file = fs::read(input_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20500,
    })?;

    if parameters.width > 0 || parameters.height > 0 {
        //TODO Resize
    }

    let compressed_image = compress_to_memory(in_file, parameters)?;
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
) -> std::result::Result<Vec<u8>, CaesiumError> {
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
    let image = lib_heif.decode(&handle, ColorSpace::Undefined, None)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20506,
        })?;

    // Scale the image
    // let small_img = image.scale(1024, 800, None)?;

    let mut context = HeifContext::new()
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20506,
        })?;
    let mut encoder = lib_heif.encoder_for_format(CompressionFormat::Hevc)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20507,
        })?;
    if parameters.optimize {
        encoder.set_quality(EncoderQuality::LossLess)
            .map_err(|e| CaesiumError {
                message: e.to_string(),
                code: 20508,
            })?;
    } else {
        encoder.set_quality(EncoderQuality::Lossy(parameters.heif.quality as u8))
            .map_err(|e| CaesiumError {
                message: e.to_string(),
                code: 20509,
            })?;
    }
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