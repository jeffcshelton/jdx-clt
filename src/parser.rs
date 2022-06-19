use std::{fmt, error::Error, env, path::PathBuf};

pub enum Command {
	Generate { input_path: PathBuf, output_path: PathBuf },
	Concatenate { input_paths: Vec<PathBuf>, output_path: PathBuf },
	Expand { input_path: PathBuf, output_path: PathBuf },
	Summarize { input_paths: Vec<PathBuf> },
	Version,
	Help,
}

#[derive(Debug)]
pub enum ParseError {
	NoArguments,
	TooManyArguments,

	UnrecognizedOption(String),
	UnrecognizedCommand(String),

	MissingInput,
	MissingOutput,
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NoArguments => write!(f, "No arguments given."),
			Self::TooManyArguments => write!(f, "Too many arguments."),
			Self::UnrecognizedOption(option) => write!(f, "Unrecognized option: '{option}'."),
			Self::UnrecognizedCommand(command) => write!(f, "Unrecognized command: '{command}'."),
			Self::MissingInput => write!(f, "Missing input path(s). Specify with option '-i'."),
			Self::MissingOutput => write!(f, "Missing output path(s). Specify with option '-o'."),
		}
	}
}

impl Error for ParseError {}

pub fn parse_arguments() -> Result<Command, ParseError> {
	let args = &mut env::args();
	_ = args.next();

	if let Some(action) = args.next() {
		let command = match action.as_str() {
			"generate" | "gen" => {
				let input_path = args.next()
					.map(|path| PathBuf::from(path))
					.ok_or(ParseError::MissingInput)?;
				
				let output_path = args.next()
					.map(|path| PathBuf::from(path))
					.ok_or(ParseError::MissingOutput)?;

				Command::Generate {
					input_path: input_path,
					output_path: output_path,
				}
			},
			"concatenate" | "concat" => {
				let mut input_paths = Vec::new();

				while let Some(arg) = args.next() {
					if arg.starts_with('-') {
						if arg == "-o" {
							break;
						} else {
							return Err(ParseError::UnrecognizedOption(arg));
						}
					}

					input_paths.push(PathBuf::from(arg));
				}

				if input_paths.is_empty() {
					return Err(ParseError::MissingInput);
				}

				let output_path = args.next()
					.map(|path| PathBuf::from(path))
					.ok_or(ParseError::MissingOutput)?;
	
				Command::Concatenate {
					input_paths: input_paths,
					output_path: output_path,
				}
			},
			"expand" | "exp" => {
				let input_path = args.next()
					.map(|path| PathBuf::from(path))
					.ok_or(ParseError::MissingInput)?;
				
				let output_path = args.next()
					.map(|path| PathBuf::from(path))
					.ok_or(ParseError::MissingOutput)?;

				Command::Expand {
					input_path: input_path,
					output_path: output_path,
				}
			},
			"summarize" | "sum" => {
				let input_paths = args
					.map(|arg| PathBuf::from(arg))
					.collect();
						
				Command::Summarize {
					input_paths: input_paths
				}
			},
			"version" | "ver" => Command::Version,
			"help" => Command::Help,
			unknown => return Err(ParseError::UnrecognizedCommand(unknown.to_string())),
		};

		if args.next().is_some() {
			return Err(ParseError::TooManyArguments);
		}

		return Ok(command);
	} else {
		return Err(ParseError::NoArguments);
	}
}
