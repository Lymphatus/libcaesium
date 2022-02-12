use dssim::Val;
use std::sync::Once;
use std::fs;

static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| {
        if fs::metadata(file).is_ok() {
            fs::remove_file(file).unwrap();
        }
    });
}

pub fn cleanup(file: &str) {
    if fs::metadata(file).is_ok() {
        fs::remove_file(file).unwrap();
    }
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
    let mut pars = libcaesium::initialize_parameters();
    pars.jpeg.quality = 100;
    libcaesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (2400, 1600));
    cleanup(output)
}

#[test]
fn compress_80() {
    let output = "tests/samples/output/compressed_80.jpg";
    initialize(output);
    let mut pars = libcaesium::initialize_parameters();
    pars.jpeg.quality = 80;
    libcaesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (2400, 1600));
    cleanup(output)
}

#[test]
fn compress_50() {
    let output = "tests/samples/output/compressed_50.jpg";
    initialize(output);
    let mut pars = libcaesium::initialize_parameters();
    pars.jpeg.quality = 50;
    libcaesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (2400, 1600));
    cleanup(output)
}

#[test]
fn compress_10() {
    let output = "tests/samples/output/compressed_10_드림캐쳐.jpg";
    initialize(output);
    let mut pars = libcaesium::initialize_parameters();
    pars.jpeg.quality = 10;
    libcaesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (2400, 1600));
    cleanup(output)
}

#[test]
fn optimize_jpeg() {
    let output = "tests/samples/output/compressed_optimized_드림캐쳐.jpg";
    initialize(output);
    let mut pars = libcaesium::initialize_parameters();
    pars.optimize = true;
    libcaesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (2400, 1600));

    //Floats error
    assert!(diff(output) < 0.001);

    cleanup(output)
}

#[test]
fn downscale_exact() {
    let output = "tests/samples/output/downscale_800_600.jpg";
    initialize(output);
    let mut pars = libcaesium::initialize_parameters();
    pars.jpeg.quality = 80;
    pars.width = 800;
    pars.height = 600;
    libcaesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (800, 600));
    cleanup(output)
}

#[test]
fn downscale_exact_optimize() {
    let output = "tests/samples/output/downscale_optimize_800_600.jpg";
    initialize(output);
    let mut pars = libcaesium::initialize_parameters();
    pars.optimize = true;
    pars.width = 800;
    pars.height = 600;
    libcaesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (800, 600));
    cleanup(output)
}