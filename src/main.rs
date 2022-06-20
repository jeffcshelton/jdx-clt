mod commands;
mod parser;
mod log;

use parser::Command;
use log::*;

const HELP_STR: &'static str = "
Usage:
  jdx <command> [options...]

Commands:
  generate (gen)          Generate a new JDX file from a directory of images.
  concatenate (concat)    Concatenate two or more JDX files into one.
  expand (exp)            Expand a JDX file into a directory of labeled PNGs.
  summarize (sum)         Summarize the data in a JDX file.
  version (ver)           Show the current version information for the JDX CLT.
  help                    Show this help page.

Options:
  -o <filename(s)...>     Specifies output file path(s).
";

const VERSION_STR: &'static str = concat!(
"\n\x1b[33;1m(JDX)\x1b[39m Jeffrey's Dataset Indexing Tool \x1b[34mv",
env!("CARGO_PKG_VERSION"),
"\x1b[0m\nCopyright \x1b[32;1mJeffrey C. Shelton\x1b[0m 2021 (MIT License)\n"
);

fn main() {
	let command = parser::parse_arguments()
		.unwrap_or_else(|error| log_fatal(error));

	match command {
		Command::Generate { input_path, output_path } => commands::generate(input_path, output_path),
		Command::Concatenate { input_paths, output_path } => commands::concatenate(input_paths, output_path),
		Command::Expand { input_path, output_path } => commands::expand(input_path, output_path),
		Command::Summarize { input_paths } => commands::summarize(input_paths),
		Command::Version => {
			println!("{}", VERSION_STR);
			Ok(())
		},
		Command::Help => {
			println!("{}", HELP_STR);
			Ok(())
		},
	}.unwrap_or_else(|error| log_fatal(error));
}
