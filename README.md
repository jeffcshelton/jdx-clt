# JDX CLT

The JDX CLT is a command line tool that makes interacting with JDX files simple and intuitive. JDX is the format that makes storing large datasets of images for machine learning easy, especially compared with directly managing directories filled with images that are difficult to maintain, modify, and store in a reasonable amount of space. JDX compresses labeled datasets into one single file that can be uniformly moved, processed, or even transformed all at once. Then, the file can be loaded into your program using the companion libraries, [libjdx](https://github.com/jeffreycshelton/libjdx) or [jdx-rust](https://github.com/jeffreycshelton/jdx-rust).

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

*Windows is not yet supported for JDX but will be supported at least by the release of v1.0.0*

## Usage

Generating a new JDX file from a directory of labeled images is simple. If you have a set of images and want to compress them into one JDX file, arrange them in the following structure, where the images are in a directory named with integers starting from zero as categories *(support for named category directories is coming soon)*. Image names do not matter as long as directories are named properly:

```
parent_directory
  |
  0 - image.png ...
  1 - image.png ...
  2 - image.png ...
  .
  .
```

Then, run this command in your terminal:

`$ jdx generate -i <parent_directory> -o <output_file>.jdx`

If you have a JDX file and want a summary of its contents, you can run this command:

`$ jdx summarize -i <input_file>.jdx`

And it will give an output like this:

```
=== input_file.jdx ===
JDX File v0.2.0
Color type: RGB (24 bits)
Image size: 28 x 28
Number of images: 2500
```

If you want a complete list of all commands and options, run `jdx help`.

## Development

Right now, the [JDX Project](https://github.com/jeffreycshelton/jdx) is under constant development and change. New features and bug fixes will be added frequently, so check back here often! Also, if you enjoy using JDX and would like to contribute to its development, please do! Contributions and issue submissions are welcome and help to make JDX a more capable format and tool.

## License

The JDX CLT is licensed under the [MIT License](LICENSE).
