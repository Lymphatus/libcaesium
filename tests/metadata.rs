use crate::cleanup::remove_compressed_test_file;
use bytes::Bytes;
use caesium::parameters::CSParameters;
use img_parts::png::Png as PartsPng;
use img_parts::{ImageEXIF, ImageICC};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Once;

mod cleanup;
static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| {
        remove_compressed_test_file(file);
    });
}

#[test]
fn jpeg_compress_80_with_metadata() {
    let output = "tests/samples/output/compressed_80_metadata.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.jpeg.quality = 80;
    pars.keep_metadata = true;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &pars,
    )
    .unwrap();
    assert!(Path::new(output).exists());
    assert!(metadata_is_equal(
        Path::new("tests/samples/uncompressed_드림캐쳐.jpg"),
        Path::new(output)
    ));
    remove_compressed_test_file(output)
}

#[test]
fn jpeg_optimize_with_metadata() {
    let output = "tests/samples/output/optimized_metadata.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.jpeg.optimize = true;
    pars.keep_metadata = true;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &pars,
    )
    .unwrap();
    assert!(Path::new(output).exists());
    assert!(metadata_is_equal(
        Path::new("tests/samples/uncompressed_드림캐쳐.jpg"),
        Path::new(output)
    ));
    remove_compressed_test_file(output)
}

#[test]
fn jpeg_resize_optimize_with_metadata() {
    let output = "tests/samples/output/resized_optimized_metadata.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.jpeg.optimize = true;
    pars.keep_metadata = true;
    pars.width = 200;
    pars.height = 200;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &pars,
    )
    .unwrap();
    assert!(Path::new(output).exists());
    assert!(metadata_is_equal(
        Path::new("tests/samples/uncompressed_드림캐쳐.jpg"),
        Path::new(output)
    ));
    remove_compressed_test_file(output)
}
//
// #[test]
// fn webp_compress_80_with_metadata() {
//     let output = "tests/samples/output/compressed_80_metadata.webp";
//     initialize(output);
//     let mut pars = CSParameters::new();
//     pars.webp.quality = 80;
//     pars.keep_metadata = true;
//     caesium::compress(
//         String::from("tests/samples/uncompressed_家.webp"),
//         String::from(output),
//         &pars,
//     )
//         .unwrap();
//     assert!(Path::new(output).exists());
//     assert!(metadata_is_equal(
//         Path::new("tests/samples/uncompressed_家.webp"),
//         Path::new(output)
//     ));
//     remove_compressed_test_file(output)
// }

#[test]
fn png_lossy_with_metadata() {
    let output = "tests/samples/output/lossy_metadata.png";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.png.quality = 80;
    pars.png.optimize = false;
    pars.keep_metadata = true;
    caesium::compress(String::from("tests/samples/metadata.png"), String::from(output), &pars).unwrap();
    assert!(Path::new(output).exists());
    assert!(png_metadata_is_equal(
        Path::new("tests/samples/metadata.png"),
        Path::new(output)
    ));
    remove_compressed_test_file(output)
}

#[test]
fn png_lossless_with_metadata() {
    let output = "tests/samples/output/lossless_metadata.png";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.png.optimize = true;
    pars.keep_metadata = true;
    caesium::compress(String::from("tests/samples/metadata.png"), String::from(output), &pars).unwrap();
    assert!(Path::new(output).exists());
    assert!(png_metadata_is_equal(
        Path::new("tests/samples/metadata.png"),
        Path::new(output)
    ));
    remove_compressed_test_file(output)
}

fn png_metadata_is_equal(input: &Path, output: &Path) -> bool {
    let in_buf = fs::read(input).unwrap();
    let out_buf = fs::read(output).unwrap();

    let in_png = PartsPng::from_bytes(Bytes::from(in_buf)).unwrap();
    let out_png = PartsPng::from_bytes(Bytes::from(out_buf)).unwrap();

    let in_iccp = in_png.icc_profile();
    let out_iccp = out_png.icc_profile();

    let in_exif = in_png.exif();
    let out_exif = out_png.exif();

    if in_iccp.is_none() && in_exif.is_none() {
        panic!("The test input image has no metadata to verify!");
    }

    in_iccp == out_iccp && in_exif == out_exif
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
