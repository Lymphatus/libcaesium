use std::ffi::CString;
use std::io;
use std::os::raw::{c_int, c_void};

use crate::CSParameters;

pub fn compress(
    input_path: String,
    output_path: String,
    parameters: CSParameters,
) -> Result<(), io::Error> {
    if parameters.width > 0 || parameters.height > 0 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "GIF resizing is not supported",
        ));
    }

    if parameters.optimize {
        lossless(input_path, output_path)
    } else {
        lossy(input_path, output_path, parameters)
    }
}

fn lossless(input_path: String, output_path: String) -> Result<(), io::Error> {
    let args: Vec<CString> = vec![
        CString::new(format!("{:?}", std::env::current_exe()))?,
        CString::new(input_path)?,
        CString::new(format!("--output={}", output_path))?,
        CString::new("--optimize=3")?,
    ];

    let argv: Vec<_> = args.iter().map(|a| a.as_ptr()).collect();

    unsafe {
        let result = gifsicle::gifsicle_main(argv.len() as _, argv.as_ptr());

        match result {
            0 => Ok(()),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "GIF optimization failed!",
            )),
        }
    }
}

pub fn lossy(
    input_path: String,
    output_path: String,
    parameters: CSParameters,
) -> Result<(), io::Error> {
    unsafe {
        let input_file = libc::fopen(
            CString::new(input_path)?.as_ptr(),
            CString::new("r")?.as_ptr(),
        );
        let output_file = libc::fopen(
            CString::new(output_path)?.as_ptr(),
            CString::new("w+")?.as_ptr(),
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
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "GIF compression failed!",
            )),
        }
    }
}
