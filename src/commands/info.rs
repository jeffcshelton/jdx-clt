pub fn version() {

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
