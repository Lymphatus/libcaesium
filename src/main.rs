use caesium::{compress_to_size, initialize_parameters};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = args[1].clone();
    let output = args[2].clone();

    let mut parameters = initialize_parameters();
    compress_to_size(input, output, &mut parameters, args[3].clone().parse().unwrap()).unwrap();
}
