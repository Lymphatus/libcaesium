use crate::cleanup::remove_compressed_test_file;
use caesium::parameters::CSParameters;
use caesium::SupportedFileTypes;
use std::path::Path;
use std::sync::Once;

mod cleanup;
static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| {
        remove_compressed_test_file(file);
    });
}

const JPEG: &str = "tests/samples/orientation.jpg";
const PNG: &str = "tests/samples/orientation.png";
const WEBP: &str = "tests/samples/orientation.webp";
const TIFF: &str = "tests/samples/orientation.tif";
const GIF: &str = "tests/samples/uncompressed_은하.gif";

fn orientation(path: &str) -> u32 {
    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(&file);
    let exif = match exif::Reader::new().read_from_container(&mut reader) {
        Ok(e) => e,
        Err(_) => return 1,
    };

    exif.get_field(exif::Tag::Orientation, exif::In::PRIMARY)
        .and_then(|f| f.value.get_uint(0))
        .unwrap_or(1)
}

fn has_tag(path: &str, tag: exif::Tag) -> bool {
    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(&file);
    match exif::Reader::new().read_from_container(&mut reader) {
        Ok(exif) => exif.get_field(tag, exif::In::PRIMARY).is_some(),
        Err(_) => false,
    }
}

fn exif_field_count(path: &str) -> usize {
    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(&file);
    match exif::Reader::new().read_from_container(&mut reader) {
        Ok(exif) => exif.fields().count(),
        Err(_) => 0,
    }
}

fn rotation_parameters() -> CSParameters {
    let mut pars = CSParameters::new();
    pars.keep_metadata = false;
    pars.keep_rotation = true;
    pars
}

#[test]
fn fixtures_are_meaningful() {
    for input in [JPEG, PNG, WEBP, TIFF] {
        assert_eq!(orientation(input), 6, "{input} has no orientation tag to preserve");
        assert!(
            has_tag(input, exif::Tag::Make),
            "{input} has no extra metadata to strip"
        );
    }
}

#[test]
fn jpeg_lossy_keeps_rotation() {
    let output = "tests/samples/output/rotation_lossy.jpg";
    initialize(output);
    let mut pars = rotation_parameters();
    pars.jpeg.quality = 80;
    caesium::compress(String::from(JPEG), String::from(output), &pars).unwrap();

    assert_eq!(orientation(output), 6);
    assert_eq!(exif_field_count(output), 1, "only the orientation tag should survive");
    assert!(!has_tag(output, exif::Tag::Make));
    remove_compressed_test_file(output)
}

#[test]
fn jpeg_lossless_keeps_rotation() {
    let output = "tests/samples/output/rotation_lossless.jpg";
    initialize(output);
    let mut pars = rotation_parameters();
    pars.jpeg.optimize = true;
    caesium::compress(String::from(JPEG), String::from(output), &pars).unwrap();

    assert_eq!(orientation(output), 6);
    assert_eq!(exif_field_count(output), 1);
    remove_compressed_test_file(output)
}

#[test]
fn jpeg_keeps_rotation_alongside_icc() {
    let output = "tests/samples/output/rotation_icc.jpg";
    initialize(output);
    let mut pars = rotation_parameters();
    pars.jpeg.preserve_icc = true;
    caesium::compress(String::from(JPEG), String::from(output), &pars).unwrap();

    assert_eq!(orientation(output), 6);
    assert_eq!(exif_field_count(output), 1);
    remove_compressed_test_file(output)
}

#[test]
fn jpeg_resize_keeps_rotation() {
    let output = "tests/samples/output/rotation_resized.jpg";
    initialize(output);
    let mut pars = rotation_parameters();
    pars.width = 80;
    pars.height = 60;
    caesium::compress(String::from(JPEG), String::from(output), &pars).unwrap();

    assert_eq!(orientation(output), 6);
    let dimensions = image::image_dimensions(output).unwrap();
    assert_eq!(dimensions, (60, 80));
    remove_compressed_test_file(output)
}

#[test]
fn jpeg_resize_width_without_rotation() {
    let output = "tests/samples/output/rotation_resized_width.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.width = 80;
    caesium::compress(String::from(JPEG), String::from(output), &pars).unwrap();

    assert_eq!(orientation(output), 1);
    let dimensions = image::image_dimensions(output).unwrap();
    assert_eq!(dimensions, (80, 60), "a dropped orientation tag must not swap the axes");
    remove_compressed_test_file(output)
}

#[test]
fn jpeg_resize_height_without_rotation() {
    let output = "tests/samples/output/rotation_resized_height.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.height = 80;
    caesium::compress(String::from(JPEG), String::from(output), &pars).unwrap();

    assert_eq!(orientation(output), 1);
    let dimensions = image::image_dimensions(output).unwrap();
    assert_eq!(
        dimensions,
        (107, 80),
        "a dropped orientation tag must not swap the axes"
    );
    remove_compressed_test_file(output)
}

#[test]
fn png_lossy_keeps_rotation() {
    let output = "tests/samples/output/rotation_lossy.png";
    initialize(output);
    let mut pars = rotation_parameters();
    pars.png.quality = 80;
    caesium::compress(String::from(PNG), String::from(output), &pars).unwrap();

    assert_eq!(orientation(output), 6);
    assert_eq!(exif_field_count(output), 1);
    assert!(!has_tag(output, exif::Tag::Make));
    remove_compressed_test_file(output)
}

#[test]
fn png_lossless_keeps_rotation() {
    let output = "tests/samples/output/rotation_lossless.png";
    initialize(output);
    let mut pars = rotation_parameters();
    pars.png.optimize = true;
    caesium::compress(String::from(PNG), String::from(output), &pars).unwrap();

    assert_eq!(orientation(output), 6);
    assert_eq!(exif_field_count(output), 1);
    assert!(!has_tag(output, exif::Tag::Make));
    remove_compressed_test_file(output)
}

#[test]
fn webp_keeps_rotation() {
    let output = "tests/samples/output/rotation.webp";
    initialize(output);
    let pars = rotation_parameters();
    caesium::compress(String::from(WEBP), String::from(output), &pars).unwrap();

    assert_eq!(orientation(output), 6);
    assert_eq!(exif_field_count(output), 1);
    assert!(!has_tag(output, exif::Tag::Make));
    remove_compressed_test_file(output)
}

#[test]
fn tiff_keeps_rotation() {
    let output = "tests/samples/output/rotation.tif";
    initialize(output);
    let pars = rotation_parameters();
    caesium::compress(String::from(TIFF), String::from(output), &pars).unwrap();

    assert_eq!(orientation(output), 6);
    assert!(!has_tag(output, exif::Tag::Make));
    remove_compressed_test_file(output)
}

#[test]
fn tiff_drops_rotation_by_default() {
    let output = "tests/samples/output/rotation_default.tif";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.keep_metadata = true;
    caesium::compress(String::from(TIFF), String::from(output), &pars).unwrap();

    assert_eq!(orientation(output), 1);
    remove_compressed_test_file(output)
}

#[test]
fn defaults_drop_rotation() {
    for (input, output) in [
        (JPEG, "tests/samples/output/rotation_default.jpg"),
        (PNG, "tests/samples/output/rotation_default.png"),
        (WEBP, "tests/samples/output/rotation_default.webp"),
    ] {
        initialize(output);
        let pars = CSParameters::new();
        assert!(!pars.keep_rotation, "keep_rotation must default to false");
        caesium::compress(String::from(input), String::from(output), &pars).unwrap();

        assert_eq!(orientation(output), 1, "{output} should have no orientation tag");
        remove_compressed_test_file(output)
    }
}

#[test]
fn keep_metadata_still_keeps_everything() {
    let output = "tests/samples/output/rotation_full_metadata.jpg";
    initialize(output);
    let mut pars = CSParameters::new();
    pars.keep_metadata = true;
    pars.keep_rotation = true;
    caesium::compress(String::from(JPEG), String::from(output), &pars).unwrap();

    assert_eq!(orientation(output), 6);
    assert!(has_tag(output, exif::Tag::Make));
    assert_eq!(exif_field_count(output), exif_field_count(JPEG));
    remove_compressed_test_file(output)
}

#[test]
fn gif_ignores_rotation() {
    let with = "tests/samples/output/rotation_on.gif";
    let without = "tests/samples/output/rotation_off.gif";
    initialize(with);
    initialize(without);

    let mut pars = CSParameters::new();
    caesium::compress(String::from(GIF), String::from(without), &pars).unwrap();
    pars.keep_rotation = true;
    caesium::compress(String::from(GIF), String::from(with), &pars).unwrap();

    assert!(Path::new(with).exists());
    assert_eq!(std::fs::read(with).unwrap(), std::fs::read(without).unwrap());
    remove_compressed_test_file(with);
    remove_compressed_test_file(without)
}

#[test]
fn convert_keeps_rotation() {
    for (format, output) in [
        (SupportedFileTypes::Png, "tests/samples/output/rotation_converted.png"),
        (SupportedFileTypes::WebP, "tests/samples/output/rotation_converted.webp"),
        (SupportedFileTypes::Tiff, "tests/samples/output/rotation_converted.tif"),
    ] {
        initialize(output);
        let pars = rotation_parameters();
        caesium::convert(String::from(JPEG), String::from(output), &pars, format).unwrap();

        assert_eq!(orientation(output), 6, "{output} lost its orientation");
        assert!(!has_tag(output, exif::Tag::Make), "{output} kept other metadata");
        assert_eq!(image::image_dimensions(output).unwrap(), (160, 120));
        remove_compressed_test_file(output)
    }
}

#[test]
fn convert_with_metadata_normalizes_rotation() {
    for (format, output) in [
        (SupportedFileTypes::Png, "tests/samples/output/rotation_baked.png"),
        (SupportedFileTypes::WebP, "tests/samples/output/rotation_baked.webp"),
    ] {
        initialize(output);
        let mut pars = CSParameters::new();
        pars.keep_metadata = true;
        caesium::convert(String::from(JPEG), String::from(output), &pars, format).unwrap();

        assert_eq!(image::image_dimensions(output).unwrap(), (120, 160));
        assert_eq!(orientation(output), 1, "{output} would be rotated twice");
        assert!(
            has_tag(output, exif::Tag::Make),
            "{output} lost the rest of its metadata"
        );
        remove_compressed_test_file(output)
    }
}
