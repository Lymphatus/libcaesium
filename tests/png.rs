use crate::cleanup::remove_compressed_test_file;
use std::sync::Once;

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
        &caesium::initialize_parameters(),
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/png"
    );
    assert_eq!(image::image_dimensions(output).unwrap(), (380, 287));
    remove_compressed_test_file(output)
}

#[test]
fn standard_compress_png_with_optimize_flag() {
    let output = "tests/samples/output/compressed_max.png";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.optimize = true;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.png"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/png"
    );
    assert_eq!(image::image_dimensions(output).unwrap(), (380, 287));
    remove_compressed_test_file(output)
}

// #[test]
// fn zopfli_compress_png() {
//     let output = "tests/samples/output/optimized.png";
//     initialize(output);
//     let mut params = caesium::initialize_parameters();
//     params.png.level = 3;
//     params.optimize = true;
//     params.png.force_zopfli = true;
//     caesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.png"),
//                       String::from(output),
//                       params)
//         .unwrap();
//     assert!(std::path::Path::new(output).exists());
//     assert_eq!(infer::get_from_path(output).unwrap().unwrap().mime_type(), "image/png");
//     assert_eq!(image::image_dimensions(output).unwrap(), (380, 287));
//     remove_compressed_test_file(output)
// }

#[test]
fn downscale_standard_compress_png() {
    let output = "tests/samples/output/downscale_compressed.png";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.width = 150;
    params.height = 150;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.png"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/png"
    );
    assert_eq!(image::image_dimensions(output).unwrap(), (150, 150));
    remove_compressed_test_file(output)
}

#[test]
fn downscale_standard_compress_png_with_optimize_flag() {
    let output = "tests/samples/output/downscale_compressed_max.png";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.width = 150;
    params.height = 150;
    params.optimize = true;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.png"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/png"
    );
    assert_eq!(image::image_dimensions(output).unwrap(), (150, 150));
    remove_compressed_test_file(output)
}

#[test]
fn downscale_zopfli_compress_png() {
    let output = "tests/samples/output/downscale_optimized.png";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.width = 150;
    params.height = 150;
    params.png.quality = 80;
    params.optimize = true;
    params.png.force_zopfli = true;
    caesium::compress(
        String::from("tests/samples/uncompressed_드림캐쳐.png"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/png"
    );
    assert_eq!(image::image_dimensions(output).unwrap(), (150, 150));
    remove_compressed_test_file(output)
}
