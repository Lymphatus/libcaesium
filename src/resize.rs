use std::io;
use std::io::{Cursor};
use image::{DynamicImage, GenericImageView};
use image::imageops::FilterType;
use image::io::Reader as ImageReader;

pub fn resize(image_buffer: Vec<u8>, width: u32, height: u32, format: image::ImageOutputFormat) -> Result<Vec<u8>, io::Error> {
    let mut image = match ImageReader::new(Cursor::new(image_buffer)).with_guessed_format()?.decode() {
        Ok(i) => i,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
    };

    let dimensions = compute_dimensions(image.width(), image.height(),width, height);
    image = image.resize_exact(dimensions.0, dimensions.1, FilterType::Lanczos3);

    let mut resized_file: Vec<u8> = vec![];
    match image.write_to(&mut resized_file, format) {
        Ok(_) => {}
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    Ok(resized_file)
}

pub fn resize_image(image: DynamicImage, width: u32, height: u32) -> Result<DynamicImage, io::Error> {
    let dimensions = compute_dimensions(image.width(), image.height(),width, height);
    let resized_image = image.resize_exact(dimensions.0, dimensions.1, FilterType::Lanczos3);

    Ok(resized_image)
}

fn compute_dimensions(original_width: u32, original_height: u32, desired_width: u32, desired_height: u32) -> (u32, u32) {
    if desired_width > 0 && desired_height > 0 {
        return (desired_width, desired_height);
    }

    let mut n_width = desired_width as f32;
    let mut n_height = desired_height as f32;
    let ratio = original_width as f32 / original_height as f32;

    if desired_height == 0 {
        n_height = (n_width / ratio).round();
    }

    if desired_width == 0 {
        n_width = (n_height * ratio).round();
    }

    (n_width as u32, n_height as u32)
}

#[test]
fn downscale_exact() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(compute_dimensions(original_width, original_height, 300, 300), (300, 300))
}

#[test]
fn same_exact() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(compute_dimensions(original_width, original_height, 800, 600), (800, 600))
}

#[test]
fn downscale_on_width() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(compute_dimensions(original_width, original_height, 750, 0), (750, 563))
}

#[test]
fn downscale_on_height() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(compute_dimensions(original_width, original_height, 0, 478), (637, 478))
}