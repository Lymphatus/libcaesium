use std::env;
use std::process::ExitCode;
use caesium::{compress, initialize_parameters};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let input = args[1].clone();
    let output = args[2].clone();

    let parameters = initialize_parameters();
    match compress(input, output, &parameters) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}
