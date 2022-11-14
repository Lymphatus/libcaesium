use image::io::Reader as ImageReader;

use oxipng::Deflaters::{Libdeflater, Zopfli};
use std::fs::File;
use std::io::Write;
use std::num::NonZeroU8;
use std::{fs, io};

use crate::resize::resize_image;
use crate::CSParameters;

pub fn compress(
    input_path: String,
    output_path: String,
    parameters: &CSParameters,
) -> Result<(), io::Error> {
    let mut original_path = input_path;
    if parameters.width > 0 || parameters.height > 0 {
        let image = match ImageReader::open(original_path)?.decode() {
            Ok(i) => i,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        };
        let image = resize_image(image, parameters.width, parameters.height)?;
        match image.save_with_format(output_path.clone(), image::ImageFormat::Png) {
            Ok(_) => {}
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        };
        original_path = output_path.clone();
    }

    let optimized_png: Vec<u8> = if parameters.optimize {
        lossless(original_path, parameters)?
    } else {
        lossy(original_path, parameters)?
    };

    let mut output_file_buffer = File::create(output_path)?;
    output_file_buffer.write_all(optimized_png.as_slice())?;

    Ok(())
}

fn lossy(input_path: String, parameters: &CSParameters) -> Result<Vec<u8>, io::Error> {
    let rgba_bitmap = match lodepng::decode32_file(input_path) {
        Ok(i) => i,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };

    let mut liq = imagequant::new();
    match liq.set_quality(0, parameters.png.quality as u8) {
        Ok(()) => {}
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    }

    let mut liq_image = match liq.new_image(
        rgba_bitmap.buffer.as_slice(),
        rgba_bitmap.width,
        rgba_bitmap.height,
        0.0,
    ) {
        Ok(i) => i,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };

    let mut quantization = match liq.quantize(&mut liq_image) {
        Ok(q) => q,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };

    let (palette, pixels) = match quantization.remapped(&mut liq_image) {
        Ok((pl, px)) => (pl, px),
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };

    let mut encoder = lodepng::Encoder::new();
    match encoder.set_palette(palette.as_slice()) {
        Ok(_) => {}
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    }
    let png_vec = match encoder.encode(pixels.as_slice(), rgba_bitmap.width, rgba_bitmap.height) {
        Ok(pv) => pv,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };

    Ok(png_vec)
}

fn lossless(input_path: String, parameters: &CSParameters) -> Result<Vec<u8>, io::Error> {
    let in_file = fs::read(input_path)?;
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
        oxipng_options.deflate = Libdeflater;
    }

    let optimized_png = match oxipng::optimize_from_memory(in_file.as_slice(), &oxipng_options) {
        Ok(o) => o,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };

    Ok(optimized_png)
}
