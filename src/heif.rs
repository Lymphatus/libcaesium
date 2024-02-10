use crate::utils::CaesiumError;
use crate::CSParameters;
use libheif_rs::{
    ColorSpace, CompressionFormat, EncoderQuality, HeifContext, HeifError, ItemId, LibHeif,
};
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::resize::compute_dimensions;

pub fn compress(
    input_path: String,
    output_path: String,
    parameters: &CSParameters,
    format: CompressionFormat,
) -> Result<(), CaesiumError> {
    let in_file = fs::read(input_path).map_err(|e| CaesiumError::new(e.to_string(), 20500))?;

    let compressed_image = compress_to_memory(in_file, parameters, format)?;
    let mut output_file_buffer =
        File::create(output_path).map_err(|e| CaesiumError::new(e.to_string(), 20501))?;

    output_file_buffer
        .write_all(compressed_image.as_slice())
        .map_err(|e| CaesiumError::new(e.to_string(), 20502))?;

    Ok(())
}

pub fn compress_to_memory(
    in_file: Vec<u8>,
    parameters: &CSParameters,
    format: CompressionFormat,
) -> Result<Vec<u8>, CaesiumError> {
    perform_compression(in_file, parameters, format).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20504,
    })
}

fn perform_compression(
    in_file: Vec<u8>,
    parameters: &CSParameters,
    format: CompressionFormat,
) -> Result<Vec<u8>, HeifError> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_bytes(in_file.as_slice())?;
    let handle = ctx.primary_image_handle()?;

    let mut exif: Vec<u8> = vec![];
    if parameters.keep_metadata {
        let mut meta_ids: Vec<ItemId> = vec![0; 1];
        handle.metadata_block_ids(&mut meta_ids, b"Exif");
        exif = handle.metadata(meta_ids[0])?;
    }

    // Decode the image
    let mut image = lib_heif.decode(&handle, ColorSpace::Undefined, None)?;

    if parameters.width > 0 || parameters.height > 0 {
        let (new_width, new_height) = compute_dimensions(image.width(), image.height(), parameters.width, parameters.height);
        let resized_image = image.scale(new_width, new_height, None)?;
        image = resized_image;
    }

    let mut context = HeifContext::new()?;
    let mut encoder = lib_heif.encoder_for_format(format)?;
    encoder.set_quality(get_quality(format, parameters))?;

    let handle = context.encode_image(&image, &mut encoder, None)?;

    if parameters.keep_metadata {
        context.add_exif_metadata(&handle, exif.as_slice())?;
    }

    let compressed_image = context.write_to_bytes()?;

    Ok(compressed_image)
}

fn get_quality(format: CompressionFormat, parameters: &CSParameters) -> EncoderQuality {
    if parameters.optimize {
        EncoderQuality::LossLess
    } else {
        let quality = match format {
            CompressionFormat::Av1 => parameters.avif.quality as u8,
            CompressionFormat::Hevc => parameters.heic.quality as u8,
            _ => 80,
        };
        EncoderQuality::Lossy(quality)
    }
}
