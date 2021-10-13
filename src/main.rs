mod commands;
mod parser;
mod log;

use jdx_rust::jdx;
use log::log_fatal;
use parser::{Command, ParseError};
use std::env;

fn main() {
    let command = parser::parse_arguments(env::args().collect())
        .unwrap_or_else(|error| log_fatal(
            match error {
                ParseError::NoArguments => "No arguments given.".to_string(),
                ParseError::InvalidCommand(command) => format!("Unrecognized command: '{}'.", command),
                ParseError::NoParameters(option) => format!("No parameters given for option '{}'.", option),
                ParseError::MissingInput => "Missing input path(s). Specify with option '-i'.".to_string(),
                ParseError::MissingOutput => "Missing output path(s). Specify with option '-o'.".to_string(),
                ParseError::MissingUnknown(option) => format!("Missing unknown option: '{}'. Please file an issue on the jdx-clt GitHub repository for maintainers to add a proper error.", option),
            }
        ));

    match command {
        Command::Generate { input, output } => commands::generate(input, output),
        Command::Concatenate { inputs, output } => commands::concatenate(inputs, output),
        Command::Expand { input, output } => commands::expand(input, output),
        Command::Summarize { inputs } => commands::summarize(inputs),
        Command::Version => Ok(commands::info::version()),
        Command::Help => Ok(commands::info::help()),
    }.unwrap_or_else(|error| log_fatal(
        match error {
            jdx::Error::OpenFile(path) => format!("Failed to open file: '{}'", path),
            jdx::Error::CloseFile(path) => format!("Failed to close file: '{}'", path),
            jdx::Error::ReadFile(path) => format!("Failed to read file: '{}'", path),
            jdx::Error::WriteFile(path) => format!("Failed to write to file: '{}'", path),
            jdx::Error::CorruptFile(path) => format!("Failed to parse corrupted file: '{}'", path),
            jdx::Error::UnequalWidths => "Datasets have unequal image widths.".to_string(),
            jdx::Error::UnequalHeights => "Datasets have unequal image heights.".to_string(),
            jdx::Error::UnequalBitDepths => "Datasets have unequal bit depths.".to_string(),
        }
    ));
}
