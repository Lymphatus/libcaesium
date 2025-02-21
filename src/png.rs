use std::fs;
use std::fs::File;
use std::io::Write;
use std::num::NonZeroU8;

use image::ImageFormat;
use oxipng::Deflaters::Zopfli;

use crate::error::CaesiumError;
use crate::resize::resize;
use crate::CSParameters;

pub fn compress(input_path: String, output_path: String, parameters: &CSParameters) -> Result<(), CaesiumError> {
    let mut in_file = fs::read(input_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20200,
    })?;

    if parameters.width > 0 || parameters.height > 0 {
        in_file = resize(in_file, parameters.width, parameters.height, ImageFormat::Png)?;
    }

    let optimized_png = compress_in_memory(in_file, parameters)?;
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

pub fn compress_in_memory(in_file: Vec<u8>, parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
    let input = if parameters.width > 0 || parameters.height > 0 {
        resize(in_file, parameters.width, parameters.height, ImageFormat::Png)?
    } else {
        in_file
    };

    let optimized_png = if parameters.optimize {
        lossless(input, parameters)?
    } else {
        lossy(input, parameters)?
    };

    Ok(optimized_png)
}

fn lossy(in_file: Vec<u8>, parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
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

    Ok(png_vec)
}

fn lossless(in_file: Vec<u8>, parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
    let mut oxipng_options = oxipng::Options::default();
    if !parameters.keep_metadata {
        oxipng_options.strip = oxipng::StripChunks::Safe;
    }

    if parameters.optimize && parameters.png.force_zopfli {
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

    let optimized_png =
        oxipng::optimize_from_memory(in_file.as_slice(), &oxipng_options).map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20210,
        })?;

    Ok(optimized_png)
}
