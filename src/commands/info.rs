use jdx_rust::jdx;

pub fn version() {
    let ver = jdx::Version::current();

    print!("
\x1b[1mJeffrey's Dataset Indexing Tool (JDX)\x1b[0m \x1b[34;1mv{}.{}.{}\x1b[0m
Copyright \x1b[1mJeffrey C. Shelton\x1b[0m 2021 (MIT License)
", ver.major, ver.minor, ver.patch);
}

pub fn help() {
    print!("
Usage:
  jdx <command> [options...]

Commands:
  generate (gen)          Generate a new JDX file from a directory of images.
  concatenate (concat)    Concatenate two or more JDX files into one.
  expand (exp)            Expand a JDX file into a directory of labeled PNGs.
  summarize               Summarize the data in a JDX file.
  version                 Show the current version information for the JDX CLT.
  help                    Show this help page.

Options:
  -i <filename(s)...>     Specifies input file path(s).
  -o <filename(s)...>     Specifies output file path(s).
");
}
