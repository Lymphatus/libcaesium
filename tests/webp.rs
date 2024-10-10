use std::sync::Once;
use caesium::parameters::CSParameters;
use crate::cleanup::remove_compressed_test_file;

mod cleanup;

static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| {
        remove_compressed_test_file(file);
    });
}

#[test]
fn compress_20() {
    let output = "tests/samples/output/compressed_20.webp";
    initialize(output);
    let mut params = CSParameters::new();
    params.webp.quality = 20;
    caesium::compress(
        String::from("tests/samples/uncompressed_家.webp"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/webp"
    );
    remove_compressed_test_file(output)
}

#[test]
fn compress_50() {
    let output = "tests/samples/output/compressed_50.webp";
    initialize(output);
    let mut params = CSParameters::new();
    params.webp.quality = 50;
    caesium::compress(
        String::from("tests/samples/uncompressed_家.webp"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/webp"
    );
    remove_compressed_test_file(output)
}

#[test]
fn compress_80() {
    let output = "tests/samples/output/compressed_80.webp";
    initialize(output);
    let mut params = CSParameters::new();
    params.webp.quality = 80;
    caesium::compress(
        String::from("tests/samples/uncompressed_家.webp"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/webp"
    );
    remove_compressed_test_file(output)
}

#[test]
fn compress_100() {
    let output = "tests/samples/output/compressed_100.webp";
    initialize(output);
    let mut params = CSParameters::new();
    params.webp.quality = 100;
    caesium::compress(
        String::from("tests/samples/uncompressed_家.webp"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/webp"
    );
    remove_compressed_test_file(output)
}

#[test]
fn optimize() {
    let output = "tests/samples/output/optimized.webp";
    initialize(output);
    let mut params = CSParameters::new();
    params.optimize = true;
    caesium::compress(
        String::from("tests/samples/uncompressed_家.webp"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/webp"
    );
    remove_compressed_test_file(output)
}

#[test]
fn downscale_compress_80() {
    let output = "tests/samples/output/downscale_compressed_80.webp";
    initialize(output);
    let mut params = CSParameters::new();
    params.webp.quality = 80;
    params.width = 150;
    params.height = 100;
    caesium::compress(
        String::from("tests/samples/uncompressed_家.webp"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/webp"
    );
    assert_eq!(image::image_dimensions(output).unwrap(), (150, 100));
    remove_compressed_test_file(output)
}

#[test]
fn downscale_optimize() {
    let output = "tests/samples/output/downscale_optimized.webp";
    initialize(output);
    let mut params = CSParameters::new();
    params.optimize = true;
    params.width = 150;
    params.height = 100;
    caesium::compress(
        String::from("tests/samples/uncompressed_家.webp"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/webp"
    );
    assert_eq!(image::image_dimensions(output).unwrap(), (150, 100));
    remove_compressed_test_file(output)
}

#[test]
fn compress_animated_80() {
    let output = "tests/samples/output/compressed_animated_80.webp";
    initialize(output);
    let mut params = CSParameters::new();
    params.webp.quality = 80;
    caesium::compress(
        String::from("tests/samples/uncompressed_animated.webp"),
        String::from(output),
        &params,
    )
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/webp"
    );
    remove_compressed_test_file(output)
}