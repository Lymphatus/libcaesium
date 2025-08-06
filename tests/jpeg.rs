use crate::cleanup::remove_compressed_test_file;
use caesium::parameters::CSParameters;
use dssim::Val;
use std::{fs::File, sync::Once};

mod cleanup;

static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| {
        remove_compressed_test_file(file);
    });
}

fn diff(compressed: &str) -> Val {
    let attr = dssim::Dssim::new();
    let orig = dssim::load_image(&attr, "tests/samples/uncompressed_드림캐쳐.jpg").unwrap();
    let comp = dssim::load_image(&attr, compressed).unwrap();
    let (diff, _) = attr.compare(&orig, comp);
    diff
}

#[test]
fn compress_100() {
    let output = "tests/samples/output/compressed_100.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.jpeg.quality = 100;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &pars,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (2400, 1600));
    remove_compressed_test_file(output)
}

#[test]
fn compress_80() {
    let output = "tests/samples/output/compressed_80.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.jpeg.quality = 80;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &pars,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (2400, 1600));
    remove_compressed_test_file(output)
}

#[test]
fn compress_50() {
    let output = "tests/samples/output/compressed_50.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.jpeg.quality = 50;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &pars,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (2400, 1600));
    remove_compressed_test_file(output)
}

#[test]
fn compress_10() {
    let output = "tests/samples/output/compressed_10_드림캐쳐.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.jpeg.quality = 10;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &pars,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (2400, 1600));
    remove_compressed_test_file(output)
}

#[test]
fn compress_corrupted_lossy() {
    let output = "tests/samples/output/corrupted_lossy.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.jpeg.quality = 50;
    assert!(caesium::compress(String::from("tests/samples/corrupted.jpg"), String::from(output), &pars,).is_err())
}

#[test]
fn optimize_jpeg() {
    let output = "tests/samples/output/compressed_optimized_드림캐쳐.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.jpeg.optimize = true;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &pars,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (2400, 1600));

    //Floats error
    assert!(diff(output) < 0.001);

    remove_compressed_test_file(output)
}

#[test]
fn compress_corrupted_lossless() {
    let output = "tests/samples/output/corrupted_lossless.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.jpeg.optimize = true;
    assert!(caesium::compress(String::from("tests/samples/corrupted.jpg"), String::from(output), &pars,).is_err());
}

#[test]
fn downscale_to_size() {
    let max_output_size = 2_000_000;
    let output = "tests/samples/output/downscale_800_600_to_size.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.width = 800;
    pars.height = 600;
    caesium::compress_to_size(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &mut pars,
        max_output_size,
        false,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert!(File::open(output).unwrap().metadata().unwrap().len() < max_output_size as u64);
    assert_eq!(image::image_dimensions(output).unwrap(), (800, 600));
    remove_compressed_test_file(output)
}

#[test]
fn downscale_exact() {
    let output = "tests/samples/output/downscale_800_600.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.jpeg.quality = 80;
    pars.width = 800;
    pars.height = 600;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &pars,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (800, 600));
    remove_compressed_test_file(output)
}

#[test]
fn downscale_exact_optimize() {
    let output = "tests/samples/output/downscale_optimize_800_600.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.jpeg.optimize = true;
    pars.width = 800;
    pars.height = 600;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &pars,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (800, 600));
    remove_compressed_test_file(output)
}
