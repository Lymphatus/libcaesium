use std::fs::File;
use std::io::{Read, Write};
use std::ops::Deref;

use bytes::Bytes;
use image::{DynamicImage, ImageBuffer};
use img_parts::{DynImage, ImageEXIF, ImageICC};
use img_parts::webp::WebP as PartsWebp;
use webp::{AnimDecoder, AnimEncoder, AnimFrame, WebPConfig};

use crate::CSParameters;
use crate::error::CaesiumError;
use crate::resize::resize_image;

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

    let compressed_image = compress_in_memory(input_data, parameters)?;

    let mut output_file = File::create(output_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20302,
    })?;

    output_file
        .write_all(&compressed_image)
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20303,
        })?;
    Ok(())
}

pub fn compress_in_memory(
    in_file: Vec<u8>,
    parameters: &CSParameters,
) -> Result<Vec<u8>, CaesiumError> {
    let mut iccp: Option<Bytes> = None;
    let mut exif: Option<Bytes> = None;

    if parameters.keep_metadata {
        (iccp, exif) = DynImage::from_bytes(in_file.clone().into())
            .map_err(|e| CaesiumError {
                message: e.to_string(),
                code: 20306,
            })?
            .map_or((None, None), |dyn_img| (dyn_img.icc_profile(), dyn_img.exif()));
    }

    let must_resize = parameters.width > 0 || parameters.height > 0;

    let anim_decoder = AnimDecoder::new(&in_file);
    let frames = anim_decoder.decode().map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20304,
    })?;
    let is_animated = frames.has_animation();

    let encoded_image_memory = if is_animated {
        let mut config = match WebPConfig::new() {
            Ok(c) => c,
            Err(_) => {
                return Err(CaesiumError {
                    message: "Cannot initialize WebP config".into(),
                    code: 20309,
                });
            }
        };
        config.lossless = if parameters.optimize { 1 } else { 0 };
        config.alpha_compression = if parameters.optimize { 0 } else { 1 };
        config.quality = parameters.webp.quality as f32;
        

        let mut images_data = vec![];
        let mut width = 0;
        let mut height = 0;

        for (i, f) in frames.into_iter().enumerate() {
            if must_resize {
                let mut dyn_image = to_dynamic_image(f);
                dyn_image = resize_image(dyn_image, parameters.width, parameters.height);
                if i == 0 {
                    width = dyn_image.width();
                    height = dyn_image.height();
                }

                images_data.push(dyn_image);
            } else {
                width = f.width();
                height = f.height();
                break;
            }
        }

        let mut encoder = AnimEncoder::new(width, height, &config);
        encoder.set_bgcolor(to_rgba(frames.bg_color));
        encoder.set_loop_count(frames.loop_count as i32);

        let mut last_ms = 0;
        for (i, f) in frames.into_iter().enumerate() {
            let delay_ms = f.get_time_ms() - last_ms;
            last_ms += delay_ms;

            if must_resize {
                if images_data.get(i).is_some() {
                    encoder.add_frame(AnimFrame::from_image(images_data.get(i).unwrap(), last_ms)
                        .map_err(|e| CaesiumError {
                            message: e.to_string(),
                            code: 20310,
                        })?);
                }
            } else {
                encoder.add_frame(f);
            }
        }

        encoder.encode()
    } else {
        let first_frame = match frames.get_frame(0) {
            None => {
                return Err(CaesiumError {
                    message: "Cannot get first frame".into(),
                    code: 20311,
                });
            }
            Some(f) => f
        };
        let mut input_image = (&first_frame).into();
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

        if parameters.optimize {
            if must_resize {
                encoder.encode(100.0)
            } else {
                //TODO With resize can throw an error
                encoder.encode_lossless()
            }
        } else {
            encoder.encode(parameters.webp.quality as f32)
        }
    };

    let encoded_image = encoded_image_memory.deref().to_vec();

    if iccp.is_some() || exif.is_some() {
        let mut image_with_metadata: Vec<u8> = vec![];
        let mut dyn_img = match PartsWebp::from_bytes(encoded_image.clone().into()) {
            Ok(d) => d,
            Err(_) => return Ok(encoded_image)
        };
        dyn_img.set_icc_profile(iccp);
        dyn_img.set_exif(exif);
        dyn_img.encoder()
            .write_to(&mut image_with_metadata)
            .map_err(|e| CaesiumError {
                message: e.to_string(),
                code: 20308,
            })?;

        Ok(image_with_metadata)
    } else {
        Ok(encoded_image)
    }
}

fn to_rgba(value: u32) -> [u8; 4] {
    [
        ((value >> 24) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        (value & 0xFF) as u8,
    ]
}

fn to_dynamic_image(frame: AnimFrame) -> DynamicImage {
    if frame.get_layout().is_alpha() {
        let image =
            ImageBuffer::from_raw(frame.width(), frame.height(), frame.get_image().to_owned())
                .expect("ImageBuffer couldn't be created");
        DynamicImage::ImageRgba8(image)
    } else {
        let image =
            ImageBuffer::from_raw(frame.width(), frame.height(), frame.get_image().to_owned())
                .expect("ImageBuffer couldn't be created");
        DynamicImage::ImageRgb8(image)
    }
}