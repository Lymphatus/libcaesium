use std::ffi::CString;
use std::io;
use std::os::raw::{c_int, c_void};
use crate::CSParameters;

pub struct Parameters {
    pub quality: u32,
}

pub fn compress(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), io::Error>
{
    unsafe {
        let input_file = libc::fopen(CString::new(input_path)?.as_ptr(), CString::new("r")?.as_ptr());
        let output_file = libc::fopen(CString::new(output_path)?.as_ptr(), CString::new("w+")?.as_ptr());
        let input_stream = gifsicle::Gif_ReadFile(input_file);
        libc::fclose(input_file);

        let padding: [*mut c_void; 7] = [std::ptr::null_mut(); 7];
        let mut loss = 0;
        if !parameters.optimize {
            loss = (100 - parameters.gif.quality) as c_int
        }

        let gc_info = gifsicle::Gif_CompressInfo {
            flags: 0,
            loss,
            padding,
        };
        let write_result = gifsicle::Gif_FullWriteFile(input_stream, &gc_info, output_file);
        libc::fclose(output_file);

        match write_result {
            1 => Ok(()),
            _ => Err(io::Error::new(io::ErrorKind::Other, "GIF compression failed!"))
        }
    }
}