use crate::cleanup::remove_compressed_test_file;
use caesium::parameters::CSParameters;
use std::fs::File;
use std::sync::Once;

mod cleanup;

static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| remove_compressed_test_file(file));
}

#[test]
fn compress_20() {
    let output = "tests/samples/output/compressed_20.gif";
    initialize(output);
    let mut params = CSParameters::new();
    params.gif.quality = 20;
    caesium::compress(
        String::from("tests/samples/uncompressed_은하.gif"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(infer::get_from_path(output).unwrap().unwrap().mime_type(), "image/gif");
    remove_compressed_test_file(output)
}

#[test]
fn compress_50() {
    let output = "tests/samples/output/compressed_50.gif";
    initialize(output);
    let mut params = CSParameters::new();
    params.gif.quality = 50;
    caesium::compress(
        String::from("tests/samples/uncompressed_은하.gif"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    remove_compressed_test_file(output)
}

#[test]
fn compress_80() {
    let output = "tests/samples/output/compressed_80.gif";
    initialize(output);
    let mut params = CSParameters::new();
    params.gif.quality = 80;
    caesium::compress(
        String::from("tests/samples/uncompressed_은하.gif"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    remove_compressed_test_file(output)
}

#[test]
fn compress_100() {
    let output = "tests/samples/output/compressed_100.gif";
    initialize(output);
    let mut params = CSParameters::new();
    params.gif.quality = 100;
    caesium::compress(
        String::from("tests/samples/uncompressed_은하.gif"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    remove_compressed_test_file(output)
}

#[test]
fn downscale() {
    let output = "tests/samples/output/downscale_to_size.gif";
    initialize(output);
    let mut params = CSParameters::new();
    params.width = 150;
    params.height = 100;
    caesium::compress(
        String::from("tests/samples/uncompressed_은하.gif"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(infer::get_from_path(output).unwrap().unwrap().mime_type(), "image/gif");
    assert_eq!(image::image_dimensions(output).unwrap(), (150, 100));
    remove_compressed_test_file(output)
}

#[test]
fn downscale_to_size() {
    let max_output_size = 100_000;
    let output = "tests/samples/output/downscale_to_size.gif";
    initialize(output);
    let mut params = CSParameters::new();
    params.width = 150;
    params.height = 100;
    caesium::compress_to_size(
        String::from("tests/samples/uncompressed_은하.gif"),
        String::from(output),
        &mut params,
        max_output_size,
        false,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(infer::get_from_path(output).unwrap().unwrap().mime_type(), "image/gif");
    assert!(File::open(output).unwrap().metadata().unwrap().len() < max_output_size as u64);
    assert_eq!(image::image_dimensions(output).unwrap(), (150, 100));
    remove_compressed_test_file(output)
}

fn animated_gif(width: u16, height: u16, frames: u16) -> Vec<u8> {
    let mut out = vec![];
    {
        let mut encoder = gif::Encoder::new(&mut out, width, height, &[]).unwrap();
        encoder.set_repeat(gif::Repeat::Infinite).unwrap();
        for f in 0..frames {
            let mut pixels = Vec::with_capacity(width as usize * height as usize * 4);
            for y in 0..height {
                for x in 0..width {
                    pixels.extend_from_slice(&[(x + f * 7) as u8, (y * 3) as u8, (x ^ y) as u8, 255]);
                }
            }
            let mut frame = gif::Frame::from_rgba_speed(width, height, &mut pixels, 30);
            frame.delay = 4;
            encoder.write_frame(&frame).unwrap();
        }
    }
    out
}

#[test]
fn compress_inside_saturated_rayon_pool() {
    use rayon::prelude::*;
    use std::sync::mpsc;
    use std::time::Duration;

    let input = animated_gif(170, 170, 20);
    let jobs = rayon::current_num_threads() * 4;
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let results: Vec<bool> = (0..jobs)
            .into_par_iter()
            .map(|_| {
                let mut params = CSParameters::new();
                params.gif.quality = 100;
                caesium::compress_in_memory(input.clone(), &params).is_ok()
            })
            .collect();
        let _ = tx.send(results);
    });

    let results = rx
        .recv_timeout(Duration::from_secs(120))
        .expect("GIF compression deadlocked inside the rayon global pool");
    assert!(results.iter().all(|ok| *ok));
}
