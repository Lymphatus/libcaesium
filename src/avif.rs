use std::fs::File;
use std::io::{Read, Write};

use bytes::Bytes;
use image::DynamicImage;
// use img_parts::avif::Avif as PartsAvif;
use img_parts::{DynImage, ImageEXIF, ImageICC};
use ravif::{Encoder, Img, RGBA8};

use crate::error::CaesiumError;
use crate::resize::resize_image;
use crate::CSParameters;

pub fn compress(input_path: String, output_path: String, parameters: &CSParameters) -> Result<(), CaesiumError> {
    let mut input_file = File::open(input_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20600,
    })?;

    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20601,
    })?;

    let compressed_image = compress_in_memory(&input_data, parameters)?;

    let mut output_file = File::create(output_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20602,
    })?;

    output_file.write_all(&compressed_image).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20603,
    })?;
    Ok(())
}

pub fn compress_in_memory(in_file: &[u8], parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
    // let mut iccp: Option<Bytes> = None;
    // let mut exif: Option<Bytes> = None;

    // if parameters.keep_metadata {
    //     (iccp, exif) = DynImage::from_bytes(in_file.to_vec().into())
    //         .map_err(|e| CaesiumError {
    //             message: e.to_string(),
    //             code: 20604,
    //         })?
    //         .map_or((None, None), |dyn_img| (dyn_img.icc_profile(), dyn_img.exif()));
    // }

    // Decode the input image
    let mut input_image = image::load_from_memory(in_file).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20605,
    })?;

    // // Apply resizing if requested
    // let must_resize = parameters.width > 0 || parameters.height > 0;
    // if must_resize {
    //     input_image = resize_image(input_image, parameters.width, parameters.height);
    // }

    // Convert to RGBA8
    let rgba_image = input_image.to_rgba8();
    let width = rgba_image.width() as usize;
    let height = rgba_image.height() as usize;

    // Convert to ravif format
    let pixels: Vec<RGBA8> = rgba_image
        .pixels()
        .map(|p| RGBA8 {
            r: p[0],
            g: p[1],
            b: p[2],
            a: p[3],
        })
        .collect();

    let img = Img::new(pixels.as_slice(), width, height);

    // Encode
    let encoder = Encoder::new()
        .with_quality(parameters.avif.quality as f32)
        .with_speed(parameters.avif.speed);

    let encoded_avif = encoder.encode_rgba(img).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20606,
    })?;

    let encoded_image = encoded_avif.avif_file;

    // // Try to add metadata if available
    // if iccp.is_some() || exif.is_some() {
    //     let mut image_with_metadata: Vec<u8> = vec![];
    //     let mut dyn_img = match PartsAvif::from_bytes(encoded_image.clone().into()) {
    //         Ok(d) => d,
    //         Err(_) => return Ok(encoded_image),
    //     };
    //     dyn_img.set_icc_profile(iccp);
    //     dyn_img.set_exif(exif);
    //     dyn_img
    //         .encoder()
    //         .write_to(&mut image_with_metadata)
    //         .map_err(|e| CaesiumError {
    //             message: e.to_string(),
    //             code: 20607,
    //         })?;
    //
    //     Ok(image_with_metadata)
    // } else {
    //
    // }

    Ok(encoded_image)
}
