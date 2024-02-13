use std::fs::File;
use std::io::{Read, Write};
use std::ops::Deref;

use crate::error::CaesiumError;
use crate::resize::resize_image;
use crate::CSParameters;

pub fn compress(
    input_path: String,
    output_path: String,
    parameters: &CSParameters,
) -> Result<(), CaesiumError> {
    let mut input_file = File::open(input_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20300,
    })?;

    let mut input_data = Vec::new();
    input_file
        .read_to_end(&mut input_data)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20301,
        })?;

    let mut output_file = File::create(output_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20302,
    })?;
    let compressed_image = compress_to_memory(input_data, parameters)?;
    output_file
        .write_all(&compressed_image)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20303,
        })?;
    Ok(())
}

pub fn compress_to_memory(
    in_file: Vec<u8>,
    parameters: &CSParameters,
) -> Result<Vec<u8>, CaesiumError> {
    let decoder = webp::Decoder::new(&in_file);
    let input_webp = match decoder.decode() {
        Some(img) => img,
        None => {
            return Err(CaesiumError {
                message: "Error decoding WebP image".to_string(),
                code: 20304,
            })
        }
    };
    let mut input_image = input_webp.to_image();
    let must_resize = parameters.width > 0 || parameters.height > 0;
    if must_resize {
        input_image = resize_image(input_image, parameters.width, parameters.height);
    }

    let encoder = match webp::Encoder::from_image(&input_image) {
        Ok(encoder) => encoder,
        Err(e) => {
            return Err(CaesiumError {
                message: e.to_string(),
                code: 20305,
            })
        }
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
