use std::{fs, io};
use std::fs::File;
use std::io::Write;
use image::ImageOutputFormat::Png;
use crate::CSParameters;
use crate::resize::resize;

pub struct Parameters {
    pub oxipng: oxipng::Options,
    pub quality: u32,
    pub force_zopfli: bool,
}

pub fn compress (input_path: String, output_path: String, parameters: CSParameters) -> Result<(), io::Error> {
    let mut in_file = fs::read(input_path)?;

    if parameters.width > 0 || parameters.height > 0 {
        in_file = resize(in_file, parameters.width, parameters.height, Png)?;
    }

    let optimized_png: Vec<u8> = if parameters.optimize {
        lossless(in_file, parameters)?
    } else {
        lossy(in_file, parameters)?
    };

    let mut output_file_buffer = File::create(output_path)?;
    output_file_buffer.write_all(optimized_png.as_slice())?;

    Ok(())
}

pub fn lossy (in_file: Vec<u8>, parameters: CSParameters) -> Result<Vec<u8>, io::Error> {
    lossless(in_file, parameters)
}

pub fn lossless(in_file: Vec<u8>, parameters: CSParameters) -> Result<Vec<u8>, io::Error> {
    let mut oxipng_options = parameters.png.oxipng;
    if !parameters.keep_metadata {
        oxipng_options.strip = oxipng::Headers::Safe;
    }

    if parameters.optimize && parameters.png.force_zopfli {
        oxipng_options.deflate = oxipng::Deflaters::Zopfli;
    } else {
        oxipng_options = oxipng::Options::from_preset(6);
    }

    let optimized_png = match oxipng::optimize_from_memory(in_file.as_slice(), &oxipng_options) {
        Ok(o) => o,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e))
    };

    Ok(optimized_png)
}