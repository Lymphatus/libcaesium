use std::{fs};
use std::fs::File;
use std::io::Write;
use std::num::NonZeroU8;

use image::ImageOutputFormat;
use oxipng::Deflaters::{Libdeflater, Zopfli};

use crate::{CaesiumError, CSParameters};
use crate::resize::resize;

pub fn compress(
    input_path: String,
    output_path: String,
    parameters: &CSParameters,
) -> Result<(), CaesiumError> {
    let mut in_file = fs::read(input_path).map_err(|e| CaesiumError { message: e.to_string(), code: 3100 })?;

    if parameters.width > 0 || parameters.height > 0 {
        in_file = resize(in_file, parameters.width, parameters.height, ImageOutputFormat::Png)?;
    }

    let optimized_png = compress_to_memory(in_file, parameters)?;
    let mut output_file_buffer = File::create(output_path).map_err(|e| CaesiumError { message: e.to_string(), code: 3102 })?;
    output_file_buffer.write_all(optimized_png.as_slice()).map_err(|e| CaesiumError { message: e.to_string(), code: 3103 })?;

    Ok(())
}

pub fn compress_to_memory(in_file: Vec<u8>, parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError>
{
    let optimized_png: Vec<u8> = if parameters.optimize {
        lossless(in_file, parameters)?
    } else {
        lossy(in_file, parameters)?
    };

    Ok(optimized_png)
}

fn lossy(in_file: Vec<u8>, parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
    let rgba_bitmap = lodepng::decode32(in_file).map_err(|e| CaesiumError { message: e.to_string(), code: 3104 })?;

    let mut liq = imagequant::new();
    liq.set_quality(0, parameters.png.quality as u8).map_err(|e| CaesiumError { message: e.to_string(), code: 3105 })?;

    let mut liq_image = liq.new_image(
        rgba_bitmap.buffer.as_slice(),
        rgba_bitmap.width,
        rgba_bitmap.height,
        0.0,
    ).map_err(|e| CaesiumError { message: e.to_string(), code: 3106 })?;

    let mut quantization = liq.quantize(&mut liq_image).map_err(|e| CaesiumError { message: e.to_string(), code: 3107 })?;

    let (palette, pixels) = quantization.remapped(&mut liq_image).map_err(|e| CaesiumError { message: e.to_string(), code: 3108 })?;

    let mut encoder = lodepng::Encoder::new();
    encoder.set_palette(palette.as_slice()).map_err(|e| CaesiumError { message: e.to_string(), code: 3112 })?;
    let png_vec = encoder.encode(pixels.as_slice(), rgba_bitmap.width, rgba_bitmap.height).map_err(|e| CaesiumError { message: e.to_string(), code: 3109 })?;

    Ok(png_vec)
}

fn lossless(in_file: Vec<u8>, parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
    let mut oxipng_options = oxipng::Options::default();
    if !parameters.keep_metadata {
        oxipng_options.strip = oxipng::Headers::Safe;
    }

    if parameters.optimize && parameters.png.force_zopfli {
        oxipng_options.deflate = Zopfli {
            iterations: NonZeroU8::new(15).unwrap(),
        };
    } else {
        oxipng_options = oxipng::Options::from_preset(3);
        oxipng_options.deflate = Libdeflater { compression: 6 };
    }

    let optimized_png = oxipng::optimize_from_memory(in_file.as_slice(), &oxipng_options).map_err(|e| CaesiumError { message: e.to_string(), code: 3110 })?;

    Ok(optimized_png)
}
