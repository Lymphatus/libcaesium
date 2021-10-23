use caesium;
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
fn compress_20() {
    let output = "tests/samples/output/compressed_20.webp";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.webp.quality = 20;
    caesium::compress(String::from("tests/samples/uncompressed_家.webp"),
                      String::from(output),
                      params)
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}

#[test]
fn compress_50() {
    let output = "tests/samples/output/compressed_50.webp";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.webp.quality = 50;
    caesium::compress(String::from("tests/samples/uncompressed_家.webp"),
                      String::from(output),
                      params)
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}

#[test]
fn compress_80() {
    let output = "tests/samples/output/compressed_80.webp";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.webp.quality = 80;
    caesium::compress(String::from("tests/samples/uncompressed_家.webp"),
                      String::from(output),
                      params)
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}

#[test]
fn compress_100() {
    let output = "tests/samples/output/compressed_100.webp";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.webp.quality = 100;
    caesium::compress(String::from("tests/samples/uncompressed_家.webp"),
                      String::from(output),
                      params)
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}

#[test]
fn optimize() {
    let output = "tests/samples/output/optimized.webp";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.optimize = true;
    caesium::compress(String::from("tests/samples/uncompressed_家.webp"),
                      String::from(output),
                      params)
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}
