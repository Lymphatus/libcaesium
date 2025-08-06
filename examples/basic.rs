use caesium::parameters::{CSParameters, JpegParameters};
use caesium::Compressor;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let mut compressor = Compressor::new();
    /*
     * This is the same as calling
     * let mut parameters = CSParameters::new()
     * parameters.jpeg.quality = 5;
     * parameters.keep_metadata = true;
     */
    let parameters = CSParameters {
        jpeg: JpegParameters {
            quality: 5,
            ..JpegParameters::new()
        },
        keep_metadata: true,
        ..CSParameters::new()
    };
    compressor.with_parameters(parameters);

    match compressor.compress_file(&args[1], &args[2]) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
