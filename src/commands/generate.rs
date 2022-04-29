use std::{path::PathBuf, fs, ffi::OsStr};
use image::{io::Reader as ImageReader, GenericImageView};
use crate::{log_fatal, log_warning};
use jdx::{Dataset, Image, Header};

pub fn generate(input: String, output: String) -> jdx::Result<()> {
	let output_path = PathBuf::from(output);

	if output_path.exists() {
		log_fatal("Invalid output path: File already exists.");
	}

	if output_path.extension() != Some(OsStr::new("jdx")) {
		log_warning("JDX files should end with the extension '.jdx'.");
	}

	let mut dataset: Option<Dataset> = None;

	for (i, class) in fs::read_dir(input)?.enumerate() {
		let class_path = class
			.unwrap()
			.path();

		let class_name = class_path
			.file_name()
			.unwrap_or_default()
			.to_string_lossy()
			.to_string();
		
		if class_name.starts_with('.') {
			continue;
		}

		if i > u16::MAX as usize {
			log_fatal("The number of classes in the dataset exceeds the maximum of 65,536.");
		}

		if let Ok(image_file_iter) = fs::read_dir(&class_path) {
			for image_file in image_file_iter {
				let image_path = image_file
					.unwrap()
					.path();
				
				let image_name = image_path
					.file_name()
					.unwrap_or_default()
					.to_string_lossy();
				
				if image_name.starts_with(".") {
					continue;
				}

				let image = ImageReader::open(&image_path)?
					.decode()
					.unwrap_or_else(|_| log_fatal(format!("Cannot decode file '{}' as an image.", image_name)));

				let image_width = u16::try_from(image.width());
				let image_height = u16::try_from(image.height());
				let bit_depth = u8::try_from(image.color().bits_per_pixel());

				if image_width.is_err() || image_height.is_err() || bit_depth.is_err() {
					log_fatal(format!("Image '{}' has dimensions that are too big. (limit 65,536 x 65,536 x 32 bits per pixel)", image_name))
				}

				let image_width = image_width.unwrap();
				let image_height = image_height.unwrap();
				let bit_depth = bit_depth.unwrap();

				if ![8, 24, 32].contains(&bit_depth) {
					log_fatal(format!("JDX does not support a bit-depth of {}. Only bit-depths of 8, 24, or 32 are supported.", bit_depth));
				}

				if dataset.is_none() {
					dataset = Some(Dataset::with_header(Header {
						version: jdx::Version::current(),
						image_width: image_width,
						image_height: image_height,
						bit_depth: bit_depth,
						image_count: 0,
						labels: Vec::new(),
					}));
				}

				dataset
					.as_mut()
					.unwrap()
					.push(Image {
						raw_data: image.as_bytes().to_vec(),
						width: image_width,
						height: image_height,
						bit_depth: bit_depth,
						label: class_name.clone(),
						label_index: i as u16,
					})?;
			}
		} else {
			log_warning(format!("Skipping file '{}': Cannot iterate over its contents.", class_name));
		}
	}

	dataset.unwrap()
		.write_to_path(output_path)?;

	Ok(())
}
