# JDX CLT

The JDX CLT is a command line tool that makes interacting with JDX files simple and intuitive. JDX is the format that makes storing large datasets of images for machine learning easy, especially compared with directly managing directories filled with images that are difficult to maintain, modify, and store in a reasonable amount of space. JDX compresses labeled datasets into one single file that can be uniformly moved, processed, or even transformed all at once. Then, the file can be loaded into your program using the companion libraries, [jdx-python](https://github.com/jeffreycshelton/jdx-python) or [jdx-rust](https://github.com/jeffreycshelton/jdx-rust).

## Install

Please download the latest version of [Rust](https://www.rust-lang.org/tools/install). Then compile and install the JDX CLT from source by cloning this repository, navigating to its directory, and typing:

`$ cargo install --path .`

From there, you should be able to type `jdx version` in your terminal or command line and see the tool's version. If that method does not work, your ~/.cargo/bin directory may not be on PATH. Either you can add it to PATH or manually move the executable to a place on PATH (like /usr/local/bin).

On macOS or Linux, you can do this by typing in the project directory:


```
$ cargo build --release
$ cp target/release/jdx /usr/local/bin
```

After running this, you should be able to use the `jdx` command without issue. ***DO NOT USE THIS METHOD IF THE RECOMMENDED METHOD ALREADY WORKED.***

## Usage

Generating a new JDX file from a directory of labeled images is simple. If you have a set of images and want to compress them into one JDX file, arrange them in the following structure, where the images are in a directory with the name of their class (category). Image names do not matter, but directory names will be stored in the JDX file as the class name.

```
parent_directory
  |
  class_one - image.png ...
  class_two - image.png ...
  class_three - image.png ...
  .
  .
```

Then, run this command in your terminal:

`$ jdx generate <parent_directory> <output_file>.jdx`

If you have JDX file(s) and want summaries of their contents, you can run this command:

`$ jdx info <input_file_1>.jdx <input_file_2>.jdx ...`

And it will give an output like this:

```
=== input_file_1.jdx ===
JDX File v0.2.0
Color type: RGB (24 bits)
Image size: 28 x 28
Number of images: 2500

=== input_file_2.jdx ===
...
```

For a complete list of all commands and options, run `jdx help`.

## Development

Right now, the [JDX Project](https://github.com/jeffreycshelton/jdx) is under constant development and change. New features and bug fixes will be added frequently, so check back here often! Also, if you enjoy using JDX and would like to contribute to its development, please do! Contributions and issue submissions are welcome and help to make JDX a more capable format and tool.

## License

The JDX CLT is licensed under the [MIT License](LICENSE).
