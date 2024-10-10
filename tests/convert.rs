use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Once;
use caesium::parameters::CSParameters;
use caesium::SupportedFileTypes;

use crate::cleanup::remove_compressed_test_file;

mod cleanup;

static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| {
        remove_compressed_test_file(file);
    });
}

#[test]
fn convert_jpg_to_png() {
    let output = "tests/samples/output/jpg.to.png";
    initialize(output);
    let mut params = CSParameters::new();
    params.keep_metadata = true;
    caesium::convert(String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
                     String::from(output),
                     &params,
                     SupportedFileTypes::Png).expect("Image converted successfully");
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/png"
    );
    assert!(metadata_is_equal(
        Path::new("tests/samples/uncompressed_드림캐쳐.jpg"),
        Path::new(output)
    ));
    remove_compressed_test_file(output)
}

#[test]
fn convert_jpg_to_webp() {
    let output = "tests/samples/output/jpg.to.webp";
    initialize(output);
    let params = CSParameters::new();
    caesium::convert(String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
                     String::from(output),
                     &params,
                     SupportedFileTypes::WebP).expect("Image converted successfully");
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/webp"
    );
    remove_compressed_test_file(output)
}

#[test]
fn convert_jpg_to_tiff() {
    let output = "tests/samples/output/jpg.to.tiff";
    initialize(output);
    let params = CSParameters::new();
    caesium::convert(String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
                     String::from(output),
                     &params,
                     SupportedFileTypes::Tiff).expect("Image converted successfully");
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/tiff"
    );
    remove_compressed_test_file(output)
}

#[test]
fn convert_png_to_jpg() {
    let output = "tests/samples/output/png.to.jpg";
    initialize(output);
    let mut params = CSParameters::new();
    params.keep_metadata = true;
    caesium::convert(String::from("tests/samples/uncompressed_드림캐쳐.png"),
                     String::from(output),
                     &params,
                     SupportedFileTypes::Jpeg).expect("Image converted successfully");
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/jpeg"
    );
    // assert!(metadata_is_equal(
    //     Path::new("tests/samples/uncompressed_드림캐쳐.png"),
    //     Path::new(output)
    // ));
    remove_compressed_test_file(output)
}

#[test]
fn convert_png_to_webp() {
    let output = "tests/samples/output/png.to.webp";
    initialize(output);
    let params = CSParameters::new();
    caesium::convert(String::from("tests/samples/uncompressed_드림캐쳐.png"),
                     String::from(output),
                     &params,
                     SupportedFileTypes::WebP).expect("Image converted successfully");
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/webp"
    );
    remove_compressed_test_file(output)
}

#[test]
fn convert_png_to_tiff() {
    let output = "tests/samples/output/png.to.tiff";
    initialize(output);
    let params = CSParameters::new();
    caesium::convert(String::from("tests/samples/uncompressed_드림캐쳐.png"),
                     String::from(output),
                     &params,
                     SupportedFileTypes::Tiff).expect("Image converted successfully");
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/tiff"
    );
    remove_compressed_test_file(output)
}

#[test]
fn convert_webp_to_jpg() {
    let output = "tests/samples/output/webp.to.jpg";
    initialize(output);
    let mut params = CSParameters::new();
    params.keep_metadata = true;
    caesium::convert(String::from("tests/samples/uncompressed_家.webp"),
                     String::from(output),
                     &params,
                     SupportedFileTypes::Jpeg).expect("Image converted successfully");
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/jpeg"
    );
    // assert!(metadata_is_equal(
    //     Path::new("tests/samples/uncompressed_家.webp"),
    //     Path::new(output)
    // ));
    remove_compressed_test_file(output)
}

#[test]
fn convert_webp_to_png() {
    let output = "tests/samples/output/webp.to.png";
    initialize(output);
    let params = CSParameters::new();
    caesium::convert(String::from("tests/samples/uncompressed_家.webp"),
                     String::from(output),
                     &params,
                     SupportedFileTypes::Png).expect("Image converted successfully");
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/png"
    );
    remove_compressed_test_file(output)
}

#[test]
fn convert_webp_to_tiff() {
    let output = "tests/samples/output/webp.to.tiff";
    initialize(output);
    let params = CSParameters::new();
    caesium::convert(String::from("tests/samples/uncompressed_家.webp"),
                     String::from(output),
                     &params,
                     SupportedFileTypes::Tiff).expect("Image converted successfully");
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/tiff"
    );
    remove_compressed_test_file(output)
}

#[test]
fn convert_tiff_to_jpg() {
    let output = "tests/samples/output/tiff.to.jpg";
    initialize(output);
    let params = CSParameters::new();
    caesium::convert(String::from("tests/samples/rgba8.tif"),
                     String::from(output),
                     &params,
                     SupportedFileTypes::Jpeg).expect("Image converted successfully");
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/jpeg"
    );
    remove_compressed_test_file(output)
}

#[test]
fn convert_tiff_to_png() {
    let output = "tests/samples/output/tiff.to.png";
    initialize(output);
    let params = CSParameters::new();
    caesium::convert(String::from("tests/samples/rgba8.tif"),
                     String::from(output),
                     &params,
                     SupportedFileTypes::Png).expect("Image converted successfully");
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/png"
    );
    remove_compressed_test_file(output)
}

#[test]
fn convert_tiff_to_webp() {
    let output = "tests/samples/output/tiff.to.webp";
    initialize(output);
    let params = CSParameters::new();
    caesium::convert(String::from("tests/samples/rgba8.tif"),
                     String::from(output),
                     &params,
                     SupportedFileTypes::WebP).expect("Image converted successfully");
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/webp"
    );
    remove_compressed_test_file(output)
}

fn extract_exif(path: &Path) -> HashMap<String, String> {
    let file = fs::File::open(path).unwrap();
    let mut bufreader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    let exif = exif_reader.read_from_container(&mut bufreader).unwrap();
    let mut exif_map = HashMap::new();
    for f in exif.fields() {
        exif_map.insert(format!("{}", f.tag), f.display_value().to_string());
    }

    exif_map
}

fn metadata_is_equal(input: &Path, output: &Path) -> bool {
    let original_exif_map = extract_exif(input);
    let compressed_exif_map = extract_exif(output);

    original_exif_map.eq(&compressed_exif_map)
}
