use caesium::{compress, initialize_parameters};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let input = args[1].clone();
    let output = args[2].clone();

    let parameters = initialize_parameters();
    match compress(input, output, &parameters) {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
