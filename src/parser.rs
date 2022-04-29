use std::{fmt, error::Error};
use crate::log_fatal;

pub enum Command {
	Generate { input: String, output: String },
	Concatenate { inputs: Vec<String>, output: String },
	Expand { input: String, output: String },
	Summarize { inputs: Vec<String> },
	Version,
	Help,
}

#[derive(Debug)]
pub enum ParseError {
	NoArguments,
	InvalidCommand(String),
	NoParameters(String),

	MissingInput,
	MissingOutput,
	MissingUnknown(String),
}

impl ParseError {
	fn missing_option(option: &str) -> Self {
		match option {
			"-i" => Self::MissingInput,
			"-o" => Self::MissingOutput,
			unknown => Self::MissingUnknown(unknown.to_string()),
		}
	}
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NoArguments => write!(f, "No arguments given."),
			Self::InvalidCommand(command) => write!(f, "Unrecognized command: '{}'.", command),
			Self::NoParameters(option) => write!(f, "No parameters given for option '{}'.", option),
			Self::MissingInput => write!(f, "Missing input path(s). Specify with option '-i'."),
			Self::MissingOutput => write!(f, "Missing output path(s). Specify with option '-o'."),
			Self::MissingUnknown(option) => write!(f, "Missing unknown option: '{}'. Please file an issue on the jdx-clt GitHub repository for maintainers to add a proper error.", option),
		}
	}
}

impl Error for ParseError {}

fn get_params(args: &Vec<String>, option: &str) -> Result<Vec<String>, ParseError> {
	for a in 2..args.len() {
		if args[a] == option {
			let param_start = a + 1;
			let mut param_end = param_start;

			while param_end < args.len() && !args[param_end].starts_with('-') {
				param_end += 1;
			}

			// Option was specified but no parameters were supplied
			if param_end == param_start {
				return Err(ParseError::NoParameters(option.to_string()))
			}

			// Returns a vector of parameters whose len() must be > 0
			return Ok(args[param_start..param_end].to_vec())
		}
	}

	Err(ParseError::missing_option(option))
}

pub fn parse_arguments(args: Vec<String>) -> Result<Command, ParseError> {
	if args.len() < 2 {
		return Err(ParseError::NoArguments)
	}

	Ok(match args[1].as_str() {
		"generate" | "gen" => {
			let inputs = get_params(&args, "-i")?;
			let outputs = get_params(&args, "-o")?;

			if inputs.len() != 1 { log_fatal("Must specify only one input path."); }
			if outputs.len() != 1 { log_fatal("Must specify only one output path."); }

			Command::Generate {
				input: inputs[0].clone(),
				output: outputs[0].clone(),
			}
		},
		"concatenate" | "concat" => {
			let inputs = get_params(&args, "-i")?;
			let outputs = get_params(&args, "-o")?;

			if inputs.len() < 2 { log_fatal("Must specify at least two input paths."); }
			if outputs.len() != 1 { log_fatal("Must specify only one output path."); }

			Command::Concatenate {
				inputs: inputs,
				output: outputs[0].clone(),
			}
		},
		"expand" | "exp" => {
			let inputs = get_params(&args, "-i")?;
			let outputs = get_params(&args, "-o")?;

			if inputs.len() != 1 { log_fatal("Must specify only one input path."); }
			if outputs.len() != 1 { log_fatal("Must specify only one output path."); }

			Command::Expand {
				input: inputs[0].clone(),
				output: outputs[0].clone(),
			}
		},
		"summarize" | "sum" => {
			Command::Summarize {
				inputs: get_params(&args, "-i")?
			}
		},
		"version" | "ver" => Command::Version,
		"help" => Command::Help,
		unknown => return Err(ParseError::InvalidCommand(unknown.to_string())),
	})
}
