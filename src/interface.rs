use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::parameters::ChromaSubsampling;
use crate::parameters::TiffCompression::{Deflate, Lzw, Packbits, Uncompressed};
use crate::{compress, compress_to_size, convert, error, CSParameters, SupportedFileTypes, TiffDeflateLevel};

#[repr(C)]
pub struct CCSParameters {
    pub keep_metadata: bool,
    pub jpeg_quality: u32,
    pub jpeg_chroma_subsampling: u32,
    pub jpeg_progressive: bool,
    pub jpeg_optimize: bool,
    pub png_quality: u32,
    pub png_optimization_level: u32,
    pub png_force_zopfli: bool,
    pub png_optimize: bool,
    pub gif_quality: u32,
    pub webp_quality: u32,
    pub webp_lossless: bool,
    pub tiff_compression: u32,
    pub tiff_deflate_level: u32,
    pub optimize: bool,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct CCSResult {
    pub success: bool,
    pub code: u32,
    pub error_message: *const c_char,
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn c_compress(
    input_path: *const c_char,
    output_path: *const c_char,
    params: CCSParameters,
) -> CCSResult {
    let parameters = c_set_parameters(params);

    c_return_result(compress(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        CStr::from_ptr(output_path).to_str().unwrap().to_string(),
        &parameters,
    ))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn c_compress_to_size(
    input_path: *const c_char,
    output_path: *const c_char,
    params: CCSParameters,
    max_output_size: usize,
    return_smallest: bool,
) -> CCSResult {
    let mut parameters = c_set_parameters(params);

    c_return_result(compress_to_size(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        CStr::from_ptr(output_path).to_str().unwrap().to_string(),
        &mut parameters,
        max_output_size,
        return_smallest,
    ))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn c_convert(
    input_path: *const c_char,
    output_path: *const c_char,
    format: SupportedFileTypes,
    params: CCSParameters,
) -> CCSResult {
    let parameters = c_set_parameters(params);

    c_return_result(convert(
        CStr::from_ptr(input_path).to_str().unwrap().to_string(),
        CStr::from_ptr(output_path).to_str().unwrap().to_string(),
        &parameters,
        format,
    ))
}

fn c_return_result(result: error::Result<()>) -> CCSResult {
    let mut error_message = CString::new("").unwrap();

    match result {
        Ok(_) => {
            let em_pointer = error_message.as_ptr();
            std::mem::forget(error_message);
            CCSResult {
                success: true,
                code: 0,
                error_message: em_pointer,
            }
        }
        Err(e) => {
            error_message = CString::new(e.to_string()).unwrap();
            let em_pointer = error_message.as_ptr();
            std::mem::forget(error_message);
            CCSResult {
                success: false,
                code: e.code,
                error_message: em_pointer,
            }
        }
    }
}

fn c_set_parameters(params: CCSParameters) -> CSParameters {
    let mut parameters = CSParameters::new();

    parameters.jpeg.quality = params.jpeg_quality;
    parameters.jpeg.progressive = params.jpeg_progressive;
    parameters.jpeg.optimize = params.jpeg_optimize;
    parameters.png.quality = params.png_quality;
    parameters.png.optimize = params.png_optimize;
    parameters.keep_metadata = params.keep_metadata;
    parameters.png.optimization_level = params.png_optimization_level as u8;
    parameters.png.force_zopfli = params.png_force_zopfli;
    parameters.gif.quality = params.gif_quality;
    parameters.webp.quality = params.webp_quality;
    parameters.webp.lossless = params.webp_lossless;
    parameters.width = params.width;
    parameters.height = params.height;

    parameters.jpeg.chroma_subsampling = match params.jpeg_chroma_subsampling {
        444 => ChromaSubsampling::CS444,
        422 => ChromaSubsampling::CS422,
        420 => ChromaSubsampling::CS420,
        411 => ChromaSubsampling::CS411,
        _ => ChromaSubsampling::Auto,
    };

    parameters.tiff.algorithm = match params.tiff_compression {
        1 => Lzw,
        2 => Deflate,
        3 => Packbits,
        _ => Uncompressed,
    };

    parameters.tiff.deflate_level = match params.tiff_deflate_level {
        1 => TiffDeflateLevel::Fast,
        6 => TiffDeflateLevel::Balanced,
        _ => TiffDeflateLevel::Best,
    };

    parameters
}
