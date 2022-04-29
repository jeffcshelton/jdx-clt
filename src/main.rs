mod commands;
mod parser;
mod log;

use log::*;

fn main() {
	use parser::Command;
	use std::env;

	let command = parser::parse_arguments(env::args().collect())
		.unwrap_or_else(|error| log_fatal(error));

	match command {
		Command::Generate { input, output } => commands::generate(input, output),
		Command::Concatenate { inputs, output } => commands::concatenate(inputs, output),
		Command::Expand { input, output } => commands::expand(input, output),
		Command::Summarize { inputs } => commands::summarize(inputs),
		Command::Version => Ok(commands::version()),
		Command::Help => Ok(commands::help()),
	}.unwrap_or_else(|error| log_fatal(error));
}
