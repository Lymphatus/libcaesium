use mozjpeg::{Decompress, Compress, ALL_MARKERS, NO_MARKERS};
use std::fs::File;
use std::io::Write;
use std::{io, mem};
use mozjpeg_sys as mjs;
use crate::CSParameters;
use std::fs;

pub struct Parameters {
    pub quality: u32,
}

pub fn compress(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), io::Error>
{
    return if parameters.optimize {
        unsafe {
            lossless(input_path, output_path, parameters)
        }
    } else {
        lossy(input_path, output_path, parameters)
    }
}

unsafe fn lossless(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), io::Error>
{
    let mut src_info: mjs::jpeg_decompress_struct = mem::zeroed();
    let mut src_err = mem::zeroed();
    let mut dst_info: mjs::jpeg_compress_struct = mem::zeroed();
    let mut dst_err = mem::zeroed();

    src_info.common.err = mjs::jpeg_std_error(&mut src_err);
    dst_info.common.err = mjs::jpeg_std_error(&mut dst_err);

    mjs::jpeg_create_decompress(&mut src_info);
    mjs::jpeg_create_compress(&mut dst_info);

    let in_file = fs::read(input_path)?;
    mjs::jpeg_mem_src(&mut src_info, in_file.as_ptr(), in_file.len() as _);

    if parameters.keep_metadata {
        mjs::jpeg_save_markers(&mut src_info, 0xFE, 0xFFFF);
        for m in 0..16 {
            mjs::jpeg_save_markers(&mut src_info, 0xE0 + m, 0xFFFF);
        }
    }

    mjs::jpeg_read_header(&mut src_info, i32::from(true));
    let src_coef_arrays = mjs::jpeg_read_coefficients(&mut src_info);
    mjs::jpeg_copy_critical_parameters(&src_info, &mut dst_info);
    let dst_coef_arrays = src_coef_arrays;

    dst_info.optimize_coding = i32::from(true);
    let mut buf = std::ptr::null_mut();
    let mut buf_size = 0;
    mjs::jpeg_mem_dest(&mut dst_info, &mut buf, &mut buf_size);
    mjs::jpeg_write_coefficients(&mut dst_info, dst_coef_arrays);

    if parameters.keep_metadata {
        let mut marker = src_info.marker_list;

        while !marker.is_null() {
        mjs::jpeg_write_marker(&mut dst_info, (*marker).marker as i32, (*marker).data, (*marker).data_length);
            marker = (*marker).next;
        }
    }

    mjs::jpeg_finish_compress(&mut dst_info);
    mjs::jpeg_destroy_compress(&mut dst_info);
    mjs::jpeg_finish_decompress(&mut src_info);
    mjs::jpeg_destroy_decompress(&mut src_info);

    let mut output_file_buffer = File::create(output_path)?;
    output_file_buffer.write_all(std::slice::from_raw_parts(buf, buf_size as usize))?;
    Ok(())
}

fn lossy(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), io::Error> {
    let markers_option = if parameters.keep_metadata { ALL_MARKERS } else { NO_MARKERS };
    let data = std::fs::read(input_path).unwrap();
    let mem_data = &data[..data.len()];
    let decompress = Decompress::with_markers(markers_option).from_mem(mem_data)?;
    let samp_factors = decompress.components()
        .iter()
        .map(|c| c.v_samp_factor)
        .collect::<Vec<_>>();

    let markers = decompress.markers();

    let color_space = decompress.color_space();
    let width = decompress.width();
    let height = decompress.height();

    let mut c_info = Compress::new(color_space);
    c_info.set_size(width, height);

    c_info.set_raw_data_in(true);
    c_info.set_quality(parameters.jpeg.quality as f32);
    c_info.set_progressive_mode();
    c_info.set_optimize_coding(true);
    c_info.set_optimize_scans(true);
    c_info.set_mem_dest();

    for (c, samp) in c_info.components_mut().iter_mut().zip(samp_factors) {
        c.v_samp_factor = samp;
        c.h_samp_factor = samp;
    }

    c_info.start_compress();

    if parameters.keep_metadata {
        markers.for_each(|marker| {
            c_info.write_marker(marker.marker, marker.data);
        });
    }

    let mut bitmaps = [&mut Vec::new(), &mut Vec::new(), &mut Vec::new()];
    let mut d_info = decompress.raw()?;
    d_info.read_raw_data(&mut bitmaps);
    d_info.finish_decompress();

    c_info.write_raw_data(&bitmaps.iter().map(|c| &c[..]).collect::<Vec<_>>());


    c_info.finish_compress();

    //data_to_vec does not return an error
    let data = c_info.data_to_vec().unwrap();

    let mut file = File::create(output_path)?;
    file.write_all(&data)?;

    Ok(())
}