use std::fs;
use std::fs::File;
use std::io::Write;
use std::num::NonZeroU8;

use crate::error::CaesiumError;
use crate::resize::resize;
use crate::CSParameters;
use image::ImageFormat;
use imagequant::RGBA;
use oxipng::Deflaters::Zopfli;

use bytes::Bytes;
use img_parts::png::Png as PartsPng;
use img_parts::{ImageEXIF, ImageICC};

pub fn compress(input_path: String, output_path: String, parameters: &CSParameters) -> Result<(), CaesiumError> {
    let mut in_file = fs::read(input_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20200,
    })?;

    if parameters.width > 0 || parameters.height > 0 {
        in_file = resize(&in_file, parameters.width, parameters.height, ImageFormat::Png)?;
    }

    let optimized_png = compress_in_memory(&in_file, parameters)?;
    let mut output_file_buffer = File::create(output_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20202,
    })?;
    output_file_buffer
        .write_all(optimized_png.as_slice())
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20203,
        })?;

    Ok(())
}

pub fn compress_in_memory(in_file: &[u8], parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
    if parameters.width > 0 || parameters.height > 0 {
        let input = resize(in_file, parameters.width, parameters.height, ImageFormat::Png)?;

        if parameters.png.optimize {
            Ok(lossless(&input, parameters)?)
        } else {
            Ok(lossy(&input, parameters)?)
        }
    } else if parameters.png.optimize {
        Ok(lossless(in_file, parameters)?)
    } else {
        Ok(lossy(in_file, parameters)?)
    }
}

fn lossy(in_file: &[u8], parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
    let (iccp, exif) = if parameters.keep_metadata {
        extract_metadata(in_file)
    } else {
        (None, None)
    };

    let rgba_bitmap = lodepng::decode32(in_file).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20204,
    })?;

    let mut liq = imagequant::new();
    liq.set_quality(0, parameters.png.quality as u8)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20205,
        })?;

    let mut liq_image = liq
        .new_image(
            rgba_bitmap.buffer.as_slice(),
            rgba_bitmap.width,
            rgba_bitmap.height,
            0.0,
        )
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20206,
        })?;

    let mut quantization = liq.quantize(&mut liq_image).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20207,
    })?;

    let (palette, pixels) = quantization.remapped(&mut liq_image).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20208,
    })?;

    let palette = palette
        .iter()
        .map(|px| {
            if px.a == 0 {
                RGBA { r: 0, g: 0, b: 0, a: 0 }
            } else {
                *px
            }
        })
        .collect::<Vec<RGBA>>();

    let mut encoder = lodepng::Encoder::new();
    encoder.set_palette(palette.as_slice()).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20212,
    })?;
    let png_vec = encoder
        .encode(pixels.as_slice(), rgba_bitmap.width, rgba_bitmap.height)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20209,
        })?;

    if parameters.keep_metadata && (iccp.is_some() || exif.is_some()) {
        if let Some(rewritten) = save_metadata(&png_vec, iccp, exif) {
            return Ok(rewritten);
        }
    }

    Ok(png_vec)
}

fn lossless(in_file: &[u8], parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
    let mut oxipng_options = oxipng::Options::default();
    if !parameters.keep_metadata {
        oxipng_options.strip = oxipng::StripChunks::Safe;
    }

    if parameters.png.optimize && parameters.png.force_zopfli {
        let mut iterations = 15;
        if in_file.len() > 2000000 {
            iterations = 5;
        }
        oxipng_options.deflate = Zopfli {
            iterations: NonZeroU8::new(iterations).unwrap(),
        };
    } else {
        let optimization_level = parameters.png.optimization_level.clamp(0, 6);
        oxipng_options = oxipng::Options::from_preset(optimization_level);
    }

    let optimized_png = oxipng::optimize_from_memory(in_file, &oxipng_options).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20210,
    })?;

    Ok(optimized_png)
}

fn extract_metadata(image: &[u8]) -> (Option<Bytes>, Option<Bytes>) {
    match PartsPng::from_bytes(Bytes::from(image.to_vec())) {
        Ok(png) => (png.icc_profile(), png.exif()),
        Err(_) => (None, None),
    }
}

fn save_metadata(image_buffer: &[u8], iccp: Option<Bytes>, exif: Option<Bytes>) -> Option<Vec<u8>> {
    let mut png = match PartsPng::from_bytes(Bytes::from(image_buffer.to_vec())) {
        Ok(p) => p,
        Err(_) => return None,
    };

    png.set_icc_profile(iccp);
    png.set_exif(exif);

    let mut output: Vec<u8> = Vec::new();
    match png.encoder().write_to(&mut output) {
        Ok(_) => Some(output),
        Err(_) => None,
    }
}
