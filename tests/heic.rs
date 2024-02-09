use std::sync::Once;
use libheif_rs::{HeifContext};

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
    let output = "tests/samples/output/compressed_20.heic";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.heic.quality = 20;
    caesium::compress(
        String::from("tests/samples/uncompressed.heic"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/heif"
    );
    remove_compressed_test_file(output)
}

#[test]
fn compress_50() {
    let output = "tests/samples/output/compressed_50.heic";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.heic.quality = 50;
    caesium::compress(
        String::from("tests/samples/uncompressed.heic"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/heif"
    );
    remove_compressed_test_file(output)
}

#[test]
fn compress_80() {
    let output = "tests/samples/output/compressed_80.heic";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.heic.quality = 80;
    caesium::compress(
        String::from("tests/samples/uncompressed.heic"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/heif"
    );
    remove_compressed_test_file(output)
}

#[test]
fn compress_100() {
    let output = "tests/samples/output/compressed_100.heic";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.heic.quality = 100;
    caesium::compress(
        String::from("tests/samples/uncompressed.heic"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/heif"
    );
    remove_compressed_test_file(output)
}

#[test]
fn optimize() {
    let output = "tests/samples/output/optimized.heic";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.optimize = true;
    caesium::compress(
        String::from("tests/samples/uncompressed.heic"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/heif"
    );
    remove_compressed_test_file(output)
}

#[test]
fn downscale_compress_80() {
    let output = "tests/samples/output/downscale_compressed_80.heic";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.heic.quality = 80;
    params.width = 350;
    params.height = 238;
    caesium::compress(
        String::from("tests/samples/uncompressed.heic"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/heif"
    );
    let ctx = HeifContext::read_from_file(output).unwrap();
    let handle = ctx.primary_image_handle().unwrap();
    assert_eq!(handle.width(), 350);
    assert_eq!(handle.height(), 238);
    remove_compressed_test_file(output)
}

#[test]
fn downscale_optimize() {
    let output = "tests/samples/output/downscale_optimized.heic";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.optimize = true;
    params.width = 350;
    params.height = 238;
    caesium::compress(
        String::from("tests/samples/uncompressed.heic"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/heif"
    );
    let ctx = HeifContext::read_from_file(output).unwrap();
    let handle = ctx.primary_image_handle().unwrap();
    assert_eq!(handle.width(), 350);
    assert_eq!(handle.height(), 238);
    remove_compressed_test_file(output)
}
