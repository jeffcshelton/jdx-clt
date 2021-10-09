mod parser;
mod log;

use log::log_fatal;
use parser::ParseError;
use std::env;

fn main() {
    let command = parser::parse_arguments(env::args().collect())
        .unwrap_or_else(|error| log_fatal(
            match error {
                ParseError::NoArguments => "No arguments given.".to_string(),
                ParseError::InvalidCommand(command) => format!("Unrecognized command: '{}'.", command),
                ParseError::MissingInput => "Missing input path(s). Specify with option '-i'.".to_string(),
                ParseError::MissingOutput => "Missing output path(s). Specify with option '-o'.".to_string(),
                ParseError::MissingUnknown(option) => format!("Missing unknown option: '{}'. Please file an issue on the jdx-clt GitHub repository for maintainers to add a proper error.", option),
            }
        ));
}
