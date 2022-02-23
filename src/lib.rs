mod utils;
mod jpeg;
mod png;
mod gif;
mod webp;
mod resize;

use std::error::Error;
use crate::utils::get_filetype;
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub struct CCSParameters {
    pub keep_metadata: bool,
    pub jpeg_quality: u32,
    pub png_level: u32,
    pub png_force_zopfli: bool,
    pub gif_quality: u32,
    pub webp_quality: u32,
    pub optimize: bool,
    pub width: u32,
    pub height: u32,
}

pub struct CSParameters {
    pub jpeg: jpeg::Parameters,
    pub png: png::Parameters,
    pub gif: gif::Parameters,
    pub webp: webp::Parameters,
    pub keep_metadata: bool,
    pub optimize: bool,
    pub width: u32,
    pub height: u32,
}

pub fn initialize_parameters() -> CSParameters
{
    let jpeg = jpeg::Parameters {
        quality: 80
    };

    let png = png::Parameters {
        oxipng: oxipng::Options::default(),
        level: 3,
        force_zopfli: false,
    };

    let gif = gif::Parameters {
        quality: 80
    };

    let webp = webp::Parameters {
        quality: 80
    };

    CSParameters {
        jpeg,
        png,
        gif,
        webp,
        keep_metadata: false,
        optimize: false,
        width: 0,
        height: 0,
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern fn c_compress(input_path: *const c_char, output_path: *const c_char, params: CCSParameters) -> bool {
    let mut parameters = initialize_parameters();
    parameters.jpeg.quality = params.jpeg_quality;
    parameters.png.level = params.png_level;
    parameters.optimize = params.optimize;
    parameters.keep_metadata = params.keep_metadata;
    parameters.png.force_zopfli = params.png_force_zopfli;
    parameters.gif.quality = params.gif_quality;
    parameters.webp.quality = params.webp_quality;
    parameters.width = params.width;
    parameters.height = params.height;

    let x = compress(CStr::from_ptr(input_path).to_str().unwrap().to_string(),
                     CStr::from_ptr(output_path).to_str().unwrap().to_string(),
                     parameters).is_ok();
    x
}

pub fn compress(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), Box<dyn Error>> {
    validate_parameters(&parameters)?;
    let file_type = get_filetype(&input_path);

    match file_type {
        utils::SupportedFileTypes::Jpeg => {
            return match jpeg::compress(input_path, output_path, parameters) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("JPEG compression error: {}", e.to_string());
                    Err(e.into())
                }
            };
        }
        utils::SupportedFileTypes::Png => {
            return match png::compress(input_path, output_path, parameters) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("PNG compression error: {}", e.to_string());
                    Err(e.into())
                }
            };
        }
        utils::SupportedFileTypes::Gif => {
            return match gif::compress(input_path, output_path, parameters) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("GIF compression error: {}", e.to_string());
                    Err(e.into())
                }
            };
        }
        utils::SupportedFileTypes::WebP => {
            return match webp::compress(input_path, output_path, parameters) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("WebP compression error: {}", e.to_string());
                    Err(e.into())
                }
            };
        }
        _ => return Err("Unknown file type".into())
    }
}

fn validate_parameters(parameters: &CSParameters) -> Result<(), Box<dyn Error>> {
    if parameters.jpeg.quality == 0 || parameters.jpeg.quality > 100 {
        return Err("Invalid JPEG quality value".into());
    }

    if parameters.png.level == 0 || parameters.png.level > 7 {
        return Err("Invalid PNG quality value".into());
    }

    if parameters.gif.quality > 100 {
        return Err("Invalid GIF quality value".into());
    }

    if parameters.webp.quality > 100 {
        return Err("Invalid WebP quality value".into());
    }

    Ok(())
}