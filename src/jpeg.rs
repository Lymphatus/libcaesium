use std::fs::File;
use std::io::Write;
use std::{io, mem};
use mozjpeg_sys::*;
use crate::CSParameters;
use std::fs;

pub struct Parameters {
    pub quality: u32,
}

pub fn compress(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), io::Error>
{
    unsafe {
        if parameters.optimize {
            lossless(input_path, output_path, parameters)
        } else {
            lossy(input_path, output_path, parameters)
        }
    }
}

unsafe fn lossless(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), io::Error> {
    let mut src_info: jpeg_decompress_struct = mem::zeroed();

    let mut src_err = mem::zeroed();
    let mut dst_info: jpeg_compress_struct = mem::zeroed();
    let mut dst_err = mem::zeroed();

    src_info.common.err = jpeg_std_error(&mut src_err);
    dst_info.common.err = jpeg_std_error(&mut dst_err);

    jpeg_create_decompress(&mut src_info);
    jpeg_create_compress(&mut dst_info);

    let in_file = fs::read(input_path)?;
    jpeg_mem_src(&mut src_info, in_file.as_ptr(), in_file.len() as _);

    if parameters.keep_metadata {
        jpeg_save_markers(&mut src_info, 0xFE, 0xFFFF);
        for m in 0..16 {
            jpeg_save_markers(&mut src_info, 0xE0 + m, 0xFFFF);
        }
    }

    jpeg_read_header(&mut src_info, i32::from(true));
    let src_coef_arrays = jpeg_read_coefficients(&mut src_info);
    jpeg_copy_critical_parameters(&src_info, &mut dst_info);
    let dst_coef_arrays = src_coef_arrays;

    dst_info.optimize_coding = i32::from(true);
    let mut buf = std::ptr::null_mut();
    let mut buf_size = 0;
    jpeg_mem_dest(&mut dst_info, &mut buf, &mut buf_size);
    jpeg_write_coefficients(&mut dst_info, dst_coef_arrays);

    if parameters.keep_metadata {
        let mut marker = src_info.marker_list;

        while !marker.is_null() {
            jpeg_write_marker(&mut dst_info, (*marker).marker as i32, (*marker).data, (*marker).data_length);
            marker = (*marker).next;
        }
    }

    jpeg_finish_compress(&mut dst_info);
    jpeg_destroy_compress(&mut dst_info);
    jpeg_finish_decompress(&mut src_info);
    jpeg_destroy_decompress(&mut src_info);

    let mut output_file_buffer = File::create(output_path)?;
    output_file_buffer.write_all(std::slice::from_raw_parts(buf, buf_size as usize))?;
    Ok(())
}

unsafe fn lossy(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), io::Error> {
    let mut src_info: jpeg_decompress_struct = mem::zeroed();
    let mut src_err = mem::zeroed();
    let mut dst_info: jpeg_compress_struct = mem::zeroed();
    let mut dst_err = mem::zeroed();


    src_info.common.err = jpeg_std_error(&mut src_err);
    dst_info.common.err = jpeg_std_error(&mut dst_err);

    jpeg_create_decompress(&mut src_info);
    jpeg_create_compress(&mut dst_info);

    let in_file = fs::read(input_path)?;
    jpeg_mem_src(&mut src_info, in_file.as_ptr(), in_file.len() as _);

    if parameters.keep_metadata {
        jpeg_save_markers(&mut src_info, 0xFE, 0xFFFF);
        for m in 0..16 {
            jpeg_save_markers(&mut src_info, 0xE0 + m, 0xFFFF);
        }
    }

    jpeg_read_header(&mut src_info, true as boolean);
    let mut buf = std::ptr::null_mut();
    let mut buf_size = 0;
    jpeg_mem_dest(&mut dst_info, &mut buf, &mut buf_size);

    let width = src_info.image_width;
    let height = src_info.image_height;
    let color_space = src_info.jpeg_color_space;
    src_info.out_color_space = color_space;
    jpeg_start_decompress(&mut src_info);
    let row_stride = src_info.image_width as usize * src_info.output_components as usize;
    let buffer_size = row_stride * src_info.image_height as usize;
    let mut buffer = vec![0u8; buffer_size];

    while src_info.output_scanline < src_info.output_height {
        let offset = src_info.output_scanline as usize * row_stride;
        let mut jsamparray = [buffer[offset..].as_mut_ptr()];
        //Crash on very first call of this function on Android
        jpeg_read_scanlines(&mut src_info, jsamparray.as_mut_ptr(), 1);
    }

    dst_info.image_width = width;
    dst_info.image_height = height;
    dst_info.in_color_space = color_space;
    dst_info.input_components = 3;
    jpeg_set_defaults(&mut dst_info);

    let row_stride = dst_info.image_width as usize * dst_info.input_components as usize;
    dst_info.dct_method = J_DCT_METHOD::JDCT_ISLOW;
    dst_info.optimize_coding = i32::from(true);
    jpeg_set_quality(&mut dst_info, parameters.jpeg.quality as i32, false as boolean);


    jpeg_start_compress(&mut dst_info, true as boolean);

    if parameters.keep_metadata {
        let mut marker = src_info.marker_list;

        while !marker.is_null() {
            jpeg_write_marker(&mut dst_info, (*marker).marker as i32, (*marker).data, (*marker).data_length);
            marker = (*marker).next;
        }
    }

    while dst_info.next_scanline < dst_info.image_height {
        let offset = dst_info.next_scanline as usize * row_stride;
        let jsamparray = [buffer[offset..].as_ptr()];
        jpeg_write_scanlines(&mut dst_info, jsamparray.as_ptr(), 1);
    }

    jpeg_finish_compress(&mut dst_info);
    jpeg_destroy_compress(&mut dst_info);
    jpeg_finish_decompress(&mut src_info);
    jpeg_destroy_decompress(&mut src_info);

    let mut output_file_buffer = File::create(output_path)?;
    output_file_buffer.write_all(std::slice::from_raw_parts(buf, buf_size as usize))?;
    Ok(())
}