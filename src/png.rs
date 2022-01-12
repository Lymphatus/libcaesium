use std::path::PathBuf;
use oxipng::{PngError};
use crate::CSParameters;

pub struct Parameters {
    pub oxipng: oxipng::Options,
    pub level: u32,
    pub force_zopfli: bool
}

pub fn compress(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), PngError> {
    let in_file = oxipng::InFile::Path(PathBuf::from(input_path));
    let out_file = oxipng::OutFile::Path(Some(PathBuf::from(output_path)));
    let mut oxipng_options = parameters.png.oxipng;

    if !parameters.keep_metadata {
        oxipng_options.strip = oxipng::Headers::Safe;
    }

    if parameters.optimize && parameters.png.force_zopfli {
        oxipng_options.deflate = oxipng::Deflaters::Zopfli;
    } else {
        oxipng_options.deflate = oxipng::Deflaters::Libdeflater;
        let mut preset = parameters.png.level - 1;
        if parameters.optimize {
           preset = 6;
        }
        oxipng_options = oxipng::Options::from_preset(preset as u8);
    }
    oxipng::optimize(&in_file, &out_file, &oxipng_options)
}