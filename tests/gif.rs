use std::sync::Once;
use crate::cleanup::remove_compressed_test_file;

mod cleanup;

static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| {
        remove_compressed_test_file(file)
    });
}

// #[test]
// fn compress_20() {
//     let output = "tests/samples/output/compressed_20.gif";
//     initialize(output);
//     let mut params = caesium::initialize_parameters();
//     params.gif.quality = 20;
//     caesium::compress(String::from("tests/samples/uncompressed_은하.gif"),
//                       String::from(output),
//                       params)
//         .unwrap();
//     assert!(std::path::Path::new(output).exists());
//     assert_eq!(infer::get_from_path(output).unwrap().unwrap().mime_type(), "image/webp");
//     cleanup(output)
// }
//
// #[test]
// fn compress_50() {
//     let output = "tests/samples/output/compressed_50.gif";
//     initialize(output);
//     let mut params = caesium::initialize_parameters();
//     params.gif.level = 50;
//     caesium::compress(String::from("tests/samples/uncompressed_은하.gif"),
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
//     let mut params = caesium::initialize_parameters();
//     params.gif.level = 80;
//     caesium::compress(String::from("tests/samples/uncompressed_은하.gif"),
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
//     let mut params = caesium::initialize_parameters();
//     params.gif.level = 100;
//     caesium::compress(String::from("tests/samples/uncompressed_은하.gif"),
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
//     let mut params = caesium::initialize_parameters();
//     params.optimize = true;
//     caesium::compress(String::from("tests/samples/uncompressed_은하.gif"),
//                       String::from(output),
//                       params)
//         .unwrap();
//     assert!(std::path::Path::new(output).exists());
//     cleanup(output)
// }
