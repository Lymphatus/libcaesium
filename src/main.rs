use caesium::{compress, initialize_parameters};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = args[1].clone();
    let output = args[2].clone();

    let parameters = initialize_parameters();
    compress(input, output, parameters).unwrap();
}