use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Once;

use crate::cleanup::remove_compressed_test_file;

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
    let mut pars = caesium::initialize_parameters();
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
    let mut pars = caesium::initialize_parameters();
    pars.optimize = true;
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
    let mut pars = caesium::initialize_parameters();
    pars.optimize = true;
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
//     let mut pars = caesium::initialize_parameters();
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
