use std::error::Error;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::utils::get_filetype;

mod gif;
mod jpeg;
mod png;
mod resize;
mod utils;
mod webp;

#[repr(C)]
pub struct CCSParameters {
    pub keep_metadata: bool,
    pub jpeg_quality: u32,
    pub png_quality: u32,
    pub png_force_zopfli: bool,
    pub gif_quality: u32,
    pub webp_quality: u32,
    pub optimize: bool,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct CCSResult {
    pub success: bool,
    pub error_message: *const c_char,
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

pub fn initialize_parameters() -> CSParameters {
    let jpeg = jpeg::Parameters { quality: 80 };

    let png = png::Parameters {
        oxipng: oxipng::Options::default(),
        quality: 80,
        force_zopfli: false,
    };

    let gif = gif::Parameters { quality: 80 };

    let webp = webp::Parameters { quality: 80 };

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
pub unsafe extern "C" fn c_compress(
    input_path: *const c_char,
    output_path: *const c_char,
    params: CCSParameters,
) -> CCSResult {
    let mut parameters = initialize_parameters();

    parameters.jpeg.quality = params.jpeg_quality;
    parameters.png.quality = params.png_quality;
    parameters.optimize = params.optimize;
    parameters.keep_metadata = params.keep_metadata;
    parameters.png.force_zopfli = params.png_force_zopfli;
    parameters.gif.quality = params.gif_quality;
    parameters.webp.quality = params.webp_quality;
    parameters.width = params.width;
    parameters.height = params.height;

    let mut error_message = CString::new("").unwrap();

    match compress(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        CStr::from_ptr(output_path).to_str().unwrap().to_string(),
        parameters,
    ) {
        Ok(_) => {
            let em_pointer = error_message.as_ptr();
            std::mem::forget(error_message);
            CCSResult {
                success: true,
                error_message: em_pointer,
            }
        }
        Err(e) => {
            error_message = CString::new(e.to_string()).unwrap();
            let em_pointer = error_message.as_ptr();
            std::mem::forget(error_message);
            CCSResult {
                success: false,
                error_message: em_pointer,
            }
        }
    }
}

pub fn compress(
    input_path: String,
    output_path: String,
    parameters: CSParameters,
) -> Result<(), Box<dyn Error>> {
    validate_parameters(&parameters)?;
    let file_type = get_filetype(&input_path);

    match file_type {
        utils::SupportedFileTypes::Jpeg => {
            jpeg::compress(input_path, output_path, parameters)?;
        }
        utils::SupportedFileTypes::Png => {
            png::compress(input_path, output_path, parameters)?;
        }
        utils::SupportedFileTypes::Gif => {
            gif::compress(input_path, output_path, parameters)?;
        }
        utils::SupportedFileTypes::WebP => {
            webp::compress(input_path, output_path, parameters)?;
        }
        _ => return Err("Unknown file type".into()),
    }

    Ok(())
}

fn validate_parameters(parameters: &CSParameters) -> Result<(), Box<dyn Error>> {
    if parameters.jpeg.quality == 0 || parameters.jpeg.quality > 100 {
        return Err("Invalid JPEG quality value".into());
    }

    if parameters.png.quality > 100 {
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
