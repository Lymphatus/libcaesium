use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::ops::Deref;

use crate::CSParameters;
use crate::resize::resize_image;

pub fn compress(
    input_path: String,
    output_path: String,
    parameters: &CSParameters,
) -> Result<(), io::Error> {
    let mut input_file = File::open(input_path)?;

    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    let mut output_file = File::create(output_path)?;
    let compressed_image = compress_to_memory(input_data, parameters)?;
    output_file.write_all(&compressed_image)?;
    Ok(())
}

pub fn compress_to_memory(in_file: Vec<u8>, parameters: &CSParameters) -> Result<Vec<u8>, io::Error>
{
    let decoder = webp::Decoder::new(&in_file);
    let input_webp = match decoder.decode() {
        Some(img) => img,
        None => return Err(io::Error::new(io::ErrorKind::Other, "WebP decode failed!")),
    };
    let mut input_image = input_webp.to_image();
    let must_resize = parameters.width > 0 || parameters.height > 0;
    if must_resize {
        input_image = resize_image(input_image, parameters.width, parameters.height)?;
    }

    let encoder = match webp::Encoder::from_image(&input_image) {
        Ok(encoder) => encoder,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };

    let encoded_image = if parameters.optimize {
        if must_resize {
            encoder.encode(100.0)
        } else {
            //TODO With resize can throw an error
            encoder.encode_lossless()
        }
    } else {
        encoder.encode(parameters.webp.quality as f32)
    };

    Ok(encoded_image.deref().to_vec())
}
