mod commands;
mod parser;
mod log;

use parser::Command;
use log::*;

fn main() {
	let command = parser::parse_arguments()
		.unwrap_or_else(|error| log_fatal(error));

	match command {
		Command::Generate { input_path, output_path } => commands::generate(input_path, output_path),
		Command::Concatenate { input_paths, output_path } => commands::concatenate(input_paths, output_path),
		Command::Expand { input_path, output_path } => commands::expand(input_path, output_path),
		Command::Summarize { input_paths } => commands::summarize(input_paths),
		Command::Version => Ok(commands::version()),
		Command::Help => Ok(commands::help()),
	}.unwrap_or_else(|error| log_fatal(error));
}
