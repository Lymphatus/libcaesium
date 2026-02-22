use crate::error::CaesiumError;
use crate::resize::compute_dimensions;
use crate::CSParameters;
use gifski::{progress, Settings};
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn compress(input_path: String, output_path: String, parameters: &CSParameters) -> Result<(), CaesiumError> {
    let in_file = fs::read(input_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20400,
    })?;

    let optimized_gif = compress_in_memory(&in_file, parameters)?;
    let mut output_file = File::create(output_path).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20402,
    })?;
    output_file
        .write_all(optimized_gif.as_slice())
        .map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20403,
        })?;

    Ok(())
}

pub fn compress_in_memory(in_file: &Vec<u8>, parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
    let compressed = lossy(in_file, parameters)?;

    Ok(compressed)
}

fn lossy(in_file: &Vec<u8>, parameters: &CSParameters) -> Result<Vec<u8>, CaesiumError> {
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::Indexed);
    let mut decoder = decoder.read_info(in_file.as_slice()).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20404,
    })?;
    let mut screen = gif_dispose::Screen::new_decoder(&decoder);
    let mut settings = Settings {
        quality: parameters.gif.quality as u8,
        repeat: decoder.repeat(),
        ..Default::default()
    };
    let old_w = decoder.width() as u32;
    let old_h = decoder.height() as u32;
    let (new_w, new_h) = if parameters.width > 0 || parameters.height > 0 {
        compute_dimensions(old_w, old_h, parameters.width, parameters.height)
    } else {
        (old_w, old_h)
    };
    settings.width = Some(new_w);
    settings.height = Some(new_h);

    let (collector, writer) = gifski::new(settings).map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20405,
    })?;

    let mut result = vec![];

    std::thread::scope(|t| -> Result<(), CaesiumError> {
        let frames_thread = t.spawn(move || -> Result<(), CaesiumError> {
            let mut i = 0;
            let mut total_delay_in_s = 0.0;
            while let Some(frame) = decoder.read_next_frame().map_err(|e| CaesiumError {
                message: e.to_string(),
                code: 20406,
            })? {
                screen.blit_frame(frame).map_err(|e| CaesiumError {
                    message: e.to_string(),
                    code: 20407,
                })?;
                let mut pixels = screen.pixels_rgba().map_buf(|b| b.to_owned());

                if new_w != old_w || new_h != old_h {
                    let mut raw_buf = Vec::with_capacity((old_w * old_h * 4) as usize);
                    for px in pixels.pixels() {
                        raw_buf.extend_from_slice(&[px.r, px.g, px.b, px.a]);
                    }

                    let img = image::RgbaImage::from_raw(old_w, old_h, raw_buf).unwrap();
                    let resized = image::imageops::resize(&img, new_w, new_h, image::imageops::FilterType::Lanczos3);

                    let mut new_buf = Vec::with_capacity((new_w * new_h) as usize);
                    for chunk in resized.chunks_exact(4) {
                        new_buf.push(gif_dispose::RGBA8 {
                            r: chunk[0],
                            g: chunk[1],
                            b: chunk[2],
                            a: chunk[3],
                        });
                    }

                    pixels = imgref::Img::new(new_buf, new_w as usize, new_h as usize);
                }

                let mut delay = frame.delay;
                if delay <= 1 {
                    delay = 10;
                }
                let delay_in_s = delay as f64 * 10.0 / 1000.0;
                collector
                    .add_frame_rgba(i, pixels, total_delay_in_s)
                    .map_err(|e| CaesiumError {
                        message: e.to_string(),
                        code: 20408,
                    })?;
                i += 1;
                total_delay_in_s += delay_in_s;
            }
            drop(collector);
            Ok(())
        });

        writer
            .write(&mut result, &mut progress::NoProgress {})
            .map_err(|e| CaesiumError {
                message: e.to_string(),
                code: 20409,
            })?;

        frames_thread.join().map_err(|_| CaesiumError {
            message: "Frame processing thread panicked".to_string(),
            code: 20410,
        })??;

        Ok(())
    })?;

    Ok(result)
}
