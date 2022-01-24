use libcaesium;
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

#[test]
fn standard_compress_png() {
    let output = "tests/samples/output/compressed.png";
    initialize(output);
    libcaesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.png"),
                      String::from(output),
                      libcaesium::initialize_parameters())
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}

#[test]
fn standard_compress_png_with_optimize_flag() {
    let output = "tests/samples/output/compressed_max.png";
    initialize(output);
    let mut params = libcaesium::initialize_parameters();
    params.optimize = true;
    libcaesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.png"),
                      String::from(output),
                      params)
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}

#[test]
fn zopfli_compress_png() {
    let output = "tests/samples/output/optimized.png";
    initialize(output);
    let mut params = libcaesium::initialize_parameters();
    params.png.level = 3;
    params.optimize = true;
    params.png.force_zopfli = true;
    libcaesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.png"),
                      String::from(output),
                      params)
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}
