use caesium::parameters::CSParameters;
use caesium::convert;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let input = args[1].clone();
    let output = args[2].clone();

    let mut parameters = CSParameters::new();
    parameters.png.quality = 60;

    match convert(input, output, &parameters, caesium::SupportedFileTypes::Png) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}
