use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod process_data;
mod writer;
extern crate env_logger;
extern crate odbc;
use dotenv::dotenv;
use std::fs::File;

pub fn process_name(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let output_file =
        File::create(config_params.output_file_path()).expect("unable to create output file");

    env_logger::init();
    dotenv().ok();
    match process_data::connect(&output_file) {
        Ok(()) => println!("Success"),
        Err(diag) => println!("Error: {}", diag),
    }
}
