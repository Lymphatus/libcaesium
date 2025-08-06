use crate::cleanup::remove_compressed_test_file;
use caesium::parameters::CSParameters;
use std::{fs::File, sync::Once};

mod cleanup;

static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| {
        remove_compressed_test_file(file);
    });
}

#[test]
fn standard_compress_png() {
    let output = "tests/samples/output/compressed.png";
    initialize(output);
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.png"),
        String::from(output),
        &CSParameters::new(),
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(infer::get_from_path(output).unwrap().unwrap().mime_type(), "image/png");
    assert_eq!(image::image_dimensions(output).unwrap(), (380, 287));
    remove_compressed_test_file(output)
}

#[test]
fn standard_compress_png_with_optimize_flag() {
    let output = "tests/samples/output/compressed_max.png";
    initialize(output);
    let mut params = CSParameters::new();
    params.png.optimize = true;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.png"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(infer::get_from_path(output).unwrap().unwrap().mime_type(), "image/png");
    assert_eq!(image::image_dimensions(output).unwrap(), (380, 287));
    remove_compressed_test_file(output)
}

#[test]
fn zopfli_compress_png() {
    let output = "tests/samples/output/optimized.png";
    initialize(output);
    let mut params = CSParameters::new();
    params.png.optimize = true;
    params.png.force_zopfli = true;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.png"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(infer::get_from_path(output).unwrap().unwrap().mime_type(), "image/png");
    assert_eq!(image::image_dimensions(output).unwrap(), (380, 287));
    remove_compressed_test_file(output)
}

#[test]
fn downscale_standard_compress_png() {
    let output = "tests/samples/output/downscale_compressed.png";
    initialize(output);
    let mut params = CSParameters::new();
    params.width = 150;
    params.height = 150;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.png"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(infer::get_from_path(output).unwrap().unwrap().mime_type(), "image/png");
    assert_eq!(image::image_dimensions(output).unwrap(), (150, 150));
    remove_compressed_test_file(output)
}

#[test]
fn downscale_standard_compress_png_with_optimize_flag() {
    let output = "tests/samples/output/downscale_compressed_max.png";
    initialize(output);
    let mut params = CSParameters::new();
    params.width = 150;
    params.height = 150;
    params.png.optimize = true;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.png"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(infer::get_from_path(output).unwrap().unwrap().mime_type(), "image/png");
    assert_eq!(image::image_dimensions(output).unwrap(), (150, 150));
    remove_compressed_test_file(output)
}

#[test]
fn downscale_zopfli_compress_png() {
    let output = "tests/samples/output/downscale_optimized.png";
    initialize(output);
    let mut params = CSParameters::new();
    params.width = 150;
    params.height = 150;
    params.png.quality = 80;
    params.png.optimize = true;
    params.png.force_zopfli = true;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.png"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(infer::get_from_path(output).unwrap().unwrap().mime_type(), "image/png");
    assert_eq!(image::image_dimensions(output).unwrap(), (150, 150));
    remove_compressed_test_file(output)
}

#[test]
fn downscale_to_size() {
    let max_output_size = 20_000;
    let output = "tests/samples/output/downscale_to_size.png";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.width = 150;
    pars.height = 150;
    caesium::compress_to_size(
        String::from("tests/samples/uncompressed_드림캐쳐.png"),
        String::from(output),
        &mut pars,
        max_output_size,
        false,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/png");
    assert!(File::open(output).unwrap().metadata().unwrap().len() < max_output_size as u64);
    assert_eq!(image::image_dimensions(output).unwrap(), (150, 150));
    remove_compressed_test_file(output)
}
