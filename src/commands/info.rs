pub fn version() {
  let version_str = option_env!("CARGO_PKG_VERSION")
    .map(|ver| "v".to_owned() + ver)
    .unwrap_or("<unknown version>".to_owned());

	println!("
\x1b[33;1m(JDX)\x1b[39m Jeffrey's Dataset Indexing Tool \x1b[34m{version_str}\x1b[0m
Copyright \x1b[32;1mJeffrey C. Shelton\x1b[0m 2021 (MIT License)
");
}

pub fn help() {
	println!("
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
");
}
