use std::io::Cursor;

use image::DynamicImage;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use crate::CaesiumError;

pub fn resize(
    image_buffer: Vec<u8>,
    width: u32,
    height: u32,
    format: image::ImageOutputFormat,
) -> Result<Vec<u8>, CaesiumError> {
    let mut image = ImageReader::new(Cursor::new(image_buffer))
        .with_guessed_format()
        .map_err(|e| CaesiumError { message: e.to_string(), code: 10300 })?
        .decode()
        .map_err(|e| CaesiumError { message: e.to_string(), code: 10301 })?;

    let dimensions = compute_dimensions(image.width(), image.height(), width, height);
    image = image.resize_exact(dimensions.0, dimensions.1, FilterType::Lanczos3);

    let mut resized_file: Vec<u8> = vec![];
    image.write_to(&mut Cursor::new(&mut resized_file), format).map_err(|e| CaesiumError { message: e.to_string(), code: 10302 })?;

    Ok(resized_file)
}

pub fn resize_image(
    image: DynamicImage,
    width: u32,
    height: u32,
) -> DynamicImage {
    let dimensions = compute_dimensions(image.width(), image.height(), width, height);
    image.resize_exact(dimensions.0, dimensions.1, FilterType::Lanczos3)
}

fn compute_dimensions(
    original_width: u32,
    original_height: u32,
    desired_width: u32,
    desired_height: u32,
) -> (u32, u32) {
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

    assert_eq!(
        compute_dimensions(original_width, original_height, 300, 300),
        (300, 300)
    )
}

#[test]
fn same_exact() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(original_width, original_height, 800, 600),
        (800, 600)
    )
}

#[test]
fn downscale_on_width() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(original_width, original_height, 750, 0),
        (750, 563)
    )
}

#[test]
fn downscale_on_height() {
    let original_width = 800;
    let original_height = 600;

    assert_eq!(
        compute_dimensions(original_width, original_height, 0, 478),
        (637, 478)
    )
}
