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
    if parameters.width > 0 || parameters.height > 0 {
        let (new_w, new_h) = compute_dimensions(
            decoder.width() as u32,
            decoder.height() as u32,
            parameters.width,
            parameters.height,
        );
        settings.width = Some(new_w);
        settings.height = Some(new_h);
    }

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
                let pixels = screen.pixels_rgba().map_buf(|b| b.to_owned());
                let delay_in_s = frame.delay as f64 * 10.0 / 1000.0;
                let timestamp = total_delay_in_s + delay_in_s;
                collector
                    .add_frame_rgba(i, pixels, timestamp)
                    .map_err(|e| CaesiumError {
                        message: e.to_string(),
                        code: 20408,
                    })?;
                i += 1;
                total_delay_in_s = timestamp;
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
        })?
    })
    .map_err(|e| CaesiumError {
        message: e.to_string(),
        code: 20411,
    })?;

    Ok(result)
}
