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

// #[test]
// fn compress_20() {
//     let output = "tests/samples/output/compressed_20.gif";
//     initialize(output);
//     let mut params = libcaesium::initialize_parameters();
//     params.gif.level = 20;
//     libcaesium::compress(String::from("tests/samples/uncompressed_은하.gif"),
//                       String::from(output),
//                       params)
//         .unwrap();
//     assert!(std::path::Path::new(output).exists());
//     cleanup(output)
// }
//
// #[test]
// fn compress_50() {
//     let output = "tests/samples/output/compressed_50.gif";
//     initialize(output);
//     let mut params = libcaesium::initialize_parameters();
//     params.gif.level = 50;
//     libcaesium::compress(String::from("tests/samples/uncompressed_은하.gif"),
//                       String::from(output),
//                       params)
//         .unwrap();
//     assert!(std::path::Path::new(output).exists());
//     cleanup(output)
// }
//
// #[test]
// fn compress_80() {
//     let output = "tests/samples/output/compressed_80.gif";
//     initialize(output);
//     let mut params = libcaesium::initialize_parameters();
//     params.gif.level = 80;
//     libcaesium::compress(String::from("tests/samples/uncompressed_은하.gif"),
//                       String::from(output),
//                       params)
//         .unwrap();
//     assert!(std::path::Path::new(output).exists());
//     cleanup(output)
// }
//
// #[test]
// fn compress_100() {
//     let output = "tests/samples/output/compressed_100.gif";
//     initialize(output);
//     let mut params = libcaesium::initialize_parameters();
//     params.gif.level = 100;
//     libcaesium::compress(String::from("tests/samples/uncompressed_은하.gif"),
//                       String::from(output),
//                       params)
//         .unwrap();
//     assert!(std::path::Path::new(output).exists());
//     cleanup(output)
// }
//
// #[test]
// fn optimize_gif() {
//     let output = "tests/samples/output/optimized.gif";
//     initialize(output);
//     let mut params = libcaesium::initialize_parameters();
//     params.optimize = true;
//     libcaesium::compress(String::from("tests/samples/uncompressed_은하.gif"),
//                       String::from(output),
//                       params)
//         .unwrap();
//     assert!(std::path::Path::new(output).exists());
//     cleanup(output)
// }
