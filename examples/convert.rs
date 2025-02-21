use caesium::convert;
use caesium::parameters::CSParameters;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let input = args[1].clone();
    let output = args[2].clone();

    let mut parameters = CSParameters::new();
    parameters.keep_metadata = true;
    parameters.webp.quality = 60;

    match convert(input, output, &parameters, caesium::SupportedFileTypes::WebP) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}
