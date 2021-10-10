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
            let mut p = a;

            while !args[p].starts_with('-') && p < args.len() {
                p += 1;
            }

            return Ok(args[a..p].to_vec())
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
