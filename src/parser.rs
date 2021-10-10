pub enum Command {
    Generate { inputs: Vec<String>, outputs: Vec<String> },
    Concatenate { inputs: Vec<String>, outputs: Vec<String> },
    Expand { inputs: Vec<String>, outputs: Vec<String> },
    Summarize { inputs: Vec<String> },
    Version,
    Help,
}

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
        "generate" | "gen" => Command::Generate { inputs: get_params(&args, "-i")?, outputs: get_params(&args, "-o")? },
        "concatenate" | "concat" => Command::Concatenate { inputs: get_params(&args, "-i")?, outputs: get_params(&args, "-o")? },
        "expand" | "exp" => Command::Expand { inputs: get_params(&args, "-i")?, outputs: get_params(&args, "-o")? },
        "summarize" => Command::Summarize { inputs: get_params(&args, "-i")? },
        "version" => Command::Version,
        "help" => Command::Help,
        unknown => return Err(ParseError::InvalidCommand(unknown.to_string())),
    })
}
