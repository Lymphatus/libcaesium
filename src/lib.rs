mod utils;
mod jpeg;
mod png;
mod gif;
mod webp;

use std::error::Error;
use crate::utils::get_filetype;
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub struct C_CSParameters {
    pub keep_metadata: bool,
    pub jpeg_quality: u32,
    pub png_level: u32,
    pub png_force_zopfli: bool,
    pub gif_quality: u32,
    pub webp_quality: u32,
    pub optimize: bool,
}

pub struct CSParameters {
    pub jpeg: jpeg::Parameters,
    pub png: png::Parameters,
    pub gif: gif::Parameters,
    pub webp: webp::Parameters,
    pub keep_metadata: bool,
    pub optimize: bool,
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
    }
}

#[no_mangle]
pub extern fn c_compress(input_path: *const c_char, output_path: *const c_char, params: C_CSParameters) -> bool {
    unsafe {
        let mut parameters = initialize_parameters();
        parameters.jpeg.quality = params.jpeg_quality;
        parameters.png.level = params.png_level - 1;
        parameters.optimize = params.optimize;
        parameters.keep_metadata = params.keep_metadata;
        parameters.png.force_zopfli = params.png_force_zopfli;
        parameters.gif.quality = params.gif_quality;
        parameters.webp.quality = params.webp_quality;

        compress(CStr::from_ptr(input_path).to_str().unwrap().to_string(),
                 CStr::from_ptr(output_path).to_str().unwrap().to_string(),
                 parameters)
            .unwrap();

        true
    }
}

pub fn compress(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), Box<dyn Error>> {
    let file_type = get_filetype(&input_path);
    if parameters.jpeg.quality == 0 || parameters.jpeg.quality > 100 {
        return Err("Invalid JPEG quality value".into());
    }

    if parameters.png.level > 6 {
        return Err("Invalid PNG quality value".into());
    }

    if parameters.gif.quality > 100 {
        return Err("Invalid GIF quality value".into());
    }

    if parameters.webp.quality > 100 {
        return Err("Invalid WebP quality value".into());
    }


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
        _ => return Err("Unknown file type".into())
    }

    Ok(())
}