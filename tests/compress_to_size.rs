use crate::cleanup::remove_compressed_test_file;
use caesium::parameters::CSParameters;
use std::fs::File;
use std::sync::Once;

mod cleanup;

static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| {
        remove_compressed_test_file(file);
    });
}

#[test]
fn compress_to_1_byte() {
    let output = "tests/samples/output/compressed_1b.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    caesium::compress_to_size(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &mut pars,
        1,
        false,
    )
    .expect_err("Cannot compress to desired quality");
    remove_compressed_test_file(output)
}

#[test]
fn compress_to_1_byte_and_return() {
    let output = "tests/samples/output/compressed_1b_return.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    caesium::compress_to_size(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &mut pars,
        1,
        true,
    )
    .unwrap();

    assert!(std::path::Path::new(output).exists());
    assert!(File::open(output).unwrap().metadata().unwrap().len() > 1);
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    remove_compressed_test_file(output)
}

#[test]
fn compress_to_10_mb() {
    let output = "tests/samples/output/compressed_10mb.jpg";
    let max_output_size = 10_000_000;
    initialize(output);
    let mut pars = CSParameters::new();
    caesium::compress_to_size(
        String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
        String::from(output),
        &mut pars,
        max_output_size,
        false,
    )
    .unwrap();

    assert_eq!(80, pars.jpeg.quality);
    assert!(std::path::Path::new(output).exists());
    assert!(File::open(output).unwrap().metadata().unwrap().len() < max_output_size as u64);
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    remove_compressed_test_file(output)
}

#[test]
fn compress_to_range() {
    let output = "tests/samples/output/compressed_range.jpg";
    let max_desired_size = 2_000_000;
    let mut max_output_size = 500_000;
    initialize(output);

    while max_output_size < max_desired_size {
        let mut pars = CSParameters::new();
        caesium::compress_to_size(
            String::from("tests/samples/uncompressed_드림캐쳐.jpg"),
            String::from(output),
            &mut pars,
            max_output_size,
            false,
        )
        .unwrap();

        assert!(std::path::Path::new(output).exists());
        assert!(File::open(output).unwrap().metadata().unwrap().len() < max_output_size as u64);
        let kind = infer::get_from_path(output).unwrap().unwrap();
        assert_eq!(kind.mime_type(), "image/jpeg");
        max_output_size += 100_000;
        remove_compressed_test_file(output);
    }
}

#[test]
fn compress_to_higher_size_and_resize() {
    let output = "tests/samples/output/compressed_10mb.jpg";
    let max_output_size = 100_000_000;
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
    assert!(File::open(output).unwrap().metadata().unwrap().len() < max_output_size as u64);
    let kind = infer::get_from_path(output).unwrap().unwrap();
    assert_eq!(kind.mime_type(), "image/jpeg");
    assert_eq!(image::image_dimensions(output).unwrap(), (800, 600));
    remove_compressed_test_file(output)
}
