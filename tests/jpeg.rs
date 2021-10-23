use caesium;
use imgref::{Img, ImgVec};
use std::path::Path;
use dssim::{RGBAPLU, ToRGBAPLU, Val};
use load_image::ImageData;
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

fn load<P: AsRef<Path>>(path: P) -> Result<ImgVec<RGBAPLU>, lodepng::Error> {
    let img = load_image::load_image(path.as_ref(), false)?;
    match img.bitmap {
        ImageData::RGB8(ref bitmap) => Ok(Img::new(bitmap.to_rgbaplu(), img.width, img.height)),
        ImageData::RGB16(ref bitmap) => Ok(Img::new(bitmap.to_rgbaplu(), img.width, img.height)),
        ImageData::RGBA8(ref bitmap) => Ok(Img::new(bitmap.to_rgbaplu(), img.width, img.height)),
        ImageData::RGBA16(ref bitmap) => Ok(Img::new(bitmap.to_rgbaplu(), img.width, img.height)),
        ImageData::GRAY8(ref bitmap) => Ok(Img::new(bitmap.to_rgbaplu(), img.width, img.height)),
        ImageData::GRAY16(ref bitmap) => Ok(Img::new(bitmap.to_rgbaplu(), img.width, img.height)),
        ImageData::GRAYA8(ref bitmap) => Ok(Img::new(bitmap.to_rgbaplu(), img.width, img.height)),
        ImageData::GRAYA16(ref bitmap) => Ok(Img::new(bitmap.to_rgbaplu(), img.width, img.height)),
    }
}

fn diff(compressed: &str) -> Val {
    let attr = dssim::Dssim::new();
    let orig = attr.create_image(&load("tests/samples/uncompressed_드림캐쳐.jpg").unwrap()).unwrap();
    let comp = attr.create_image(&load(compressed).unwrap()).unwrap();
    let (diff, _) = attr.compare(&orig, comp);
    diff
}

#[test]
fn compress_100() {
    let output = "tests/samples/output/compressed_100.jpg";
    initialize(output);
    let mut pars = caesium::initialize_parameters();
    pars.jpeg.quality = 100;
    caesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}

#[test]
fn compress_80() {
    let output = "tests/samples/output/compressed_80.jpg";
    initialize(output);
    let mut pars = caesium::initialize_parameters();
    pars.jpeg.quality = 80;
    caesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}

#[test]
fn compress_50() {
    let output = "tests/samples/output/compressed_50.jpg";
    initialize(output);
    let mut pars = caesium::initialize_parameters();
    pars.jpeg.quality = 50;
    caesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}

#[test]
fn compress_10() {
    let output = "tests/samples/output/compressed_10_드림캐쳐.jpg";
    initialize(output);
    let mut pars = caesium::initialize_parameters();
    pars.jpeg.quality = 10;
    caesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}

#[test]
fn optimize_jpeg() {
    let output = "tests/samples/output/compressed_optimized_드림캐쳐.jpg";
    initialize(output);
    let mut pars = caesium::initialize_parameters();
    pars.optimize = true;
    caesium::compress(String::from("tests/samples/uncompressed_드림캐쳐.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());

    //Floats error
    assert!(diff(output) < 0.001);
    cleanup(output)
}