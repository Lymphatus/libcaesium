use std::fs;

pub fn remove_compressed_test_file(file: &str) {
    if fs::metadata(file).is_ok() {
        fs::remove_file(file).unwrap();
    }
}