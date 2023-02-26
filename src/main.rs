use caesium::{compress, compress_to_size, initialize_parameters};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = args[1].clone();
    let output = args[2].clone();

    let mut parameters = initialize_parameters();
    parameters.output_size = args[3].clone().parse().unwrap();
    compress_to_size(input, &mut parameters).unwrap();
    // compress(input, output, &parameters).unwrap();
}
