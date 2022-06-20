use std::{path::PathBuf, ffi::OsStr};
use crate::log_warning;
use jdx::Header;

pub fn info(input_paths: Vec<PathBuf>) -> jdx::Result<()> {
	for path in input_paths {
		println!();

		let header = Header::read_from_path(&path)?;
		let color_text = match header.bit_depth {
			8 => "\x1b[38;5;8;1mGRAY\x1b[0m",
			24 => "\x1b[1;31mR\x1b[32mG\x1b[34mB\x1b[0m",
			32 => "\x1b[1;31mR\x1b[32mG\x1b[34mB\x1b[37mA\x1b[0m",
			_ => Err(jdx::Error::CorruptFile)?
		};

		println!("=== \x1b[33;1m{}\x1b[0m ===", path.to_string_lossy());
		println!("JDX File \x1b[34m{}\x1b[0m", header.version.to_string());
		println!("Color type: {} ({} bits)", color_text, header.bit_depth);
		println!("Image size: \x1b[1m{}\x1b[0m x \x1b[1m{}\x1b[0m", header.image_width, header.image_height);
		println!("Number of images: \x1b[1m{}\x1b[0m", header.image_count);
		println!("Classes: \x1b[1m{:?}\x1b[0m", header.classes);

		if path.extension().and_then(OsStr::to_str) != Some("jdx") {
			log_warning("JDX files should end with extension '.jdx'.");
		}
	}

	println!();
	Ok(())
}
