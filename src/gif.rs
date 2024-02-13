use std::ffi::CString;
use std::os::raw::{c_int, c_void};

use crate::error::CaesiumError;
use crate::CSParameters;

pub fn compress(
    input_path: String,
    output_path: String,
    parameters: &CSParameters,
) -> Result<(), CaesiumError> {
    if parameters.width > 0 || parameters.height > 0 {
        return Err(CaesiumError {
            message: "GIF resizing is not supported".to_string(),
            code: 20400,
        });
    }

    if parameters.optimize {
        lossless(input_path, output_path)
    } else {
        lossy(input_path, output_path, parameters)
    }
}

fn lossless(input_path: String, output_path: String) -> Result<(), CaesiumError> {
    let args: Vec<CString> = vec![
        CString::new(format!("{:?}", std::env::current_exe())).map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20401,
        })?,
        CString::new(input_path).map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20402,
        })?,
        CString::new(format!("--output={}", output_path)).map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20403,
        })?,
        CString::new("--optimize=3").map_err(|e| CaesiumError {
            message: e.to_string(),
            code: 20404,
        })?,
    ];

    let argv: Vec<_> = args.iter().map(|a| a.as_ptr()).collect();

    unsafe {
        let result = gifsicle::gifsicle_main(argv.len() as _, argv.as_ptr());

        match result {
            0 => Ok(()),
            _ => Err(CaesiumError {
                message: "GIF optimization failed!".to_string(),
                code: 20405,
            }),
        }
    }
}

pub fn lossy(
    input_path: String,
    output_path: String,
    parameters: &CSParameters,
) -> Result<(), CaesiumError> {
    unsafe {
        let input_file = libc::fopen(
            CString::new(input_path)
                .map_err(|e| CaesiumError {
                    message: e.to_string(),
                    code: 20406,
                })?
                .as_ptr(),
            CString::new("r")
                .map_err(|e| CaesiumError {
                    message: e.to_string(),
                    code: 20407,
                })?
                .as_ptr(),
        );
        let output_file = libc::fopen(
            CString::new(output_path)
                .map_err(|e| CaesiumError {
                    message: e.to_string(),
                    code: 20408,
                })?
                .as_ptr(),
            CString::new("w+")
                .map_err(|e| CaesiumError {
                    message: e.to_string(),
                    code: 20409,
                })?
                .as_ptr(),
        );
        let input_stream = gifsicle::Gif_ReadFile(input_file);
        libc::fclose(input_file);

        let padding: [*mut c_void; 7] = [std::ptr::null_mut(); 7];
        let loss = (100 - parameters.gif.quality) as c_int;

        let gc_info = gifsicle::Gif_CompressInfo {
            flags: 0,
            loss,
            padding,
        };
        let write_result = gifsicle::Gif_FullWriteFile(input_stream, &gc_info, output_file);
        libc::fclose(output_file);

        match write_result {
            1 => Ok(()),
            _ => Err(CaesiumError {
                message: "GIF optimization failed!".to_string(),
                code: 20410,
            }),
        }
    }
}
