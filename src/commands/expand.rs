use image::{GrayImage, ImageFormat, RgbImage, RgbaImage};
use std::{path::PathBuf, fs};
use crate::log_fatal;
use jdx::Dataset;

pub fn expand(input: String, output: String) -> jdx::Result<()> {
	let dataset = Dataset::read_from_path(input)?;
	let output_path = PathBuf::from(output);

	if output_path.exists() {
		log_fatal("Invalid output path: Directory already exists.");
	}

	if fs::create_dir(&output_path).is_err() {
		log_fatal("Failed to create output directory.");
	}

	let bit_depth = dataset.header().bit_depth;

	for (i, image) in dataset.iter().enumerate() {
		let label_path = output_path.join(image.label);
		let image_path = label_path.join(i.to_string() + ".png");

		if !label_path.exists() {
			fs::create_dir(&label_path)?;
		}

		if bit_depth == 8 {
			let image = GrayImage::from_raw(
				image.width as u32,
				image.height as u32,
				image.raw_data.to_vec()
			).unwrap();

			if image.save_with_format(&image_path, ImageFormat::Png).is_err() {
				log_fatal("Failed to save image to path.");
			}
		} else if bit_depth == 24 {
			let image = RgbImage::from_raw(
				image.width as u32,
				image.height as u32,
				image.raw_data.to_vec(),
			).unwrap();

			if image.save_with_format(&image_path, ImageFormat::Png).is_err() {
				log_fatal("Failed to save image to path.");
			}
		} else if bit_depth == 32 {
			let image = RgbaImage::from_raw(
				image.width as u32,
				image.height as u32,
				image.raw_data.to_vec(),
			).unwrap();

			if image.save_with_format(&image_path, ImageFormat::Png).is_err() {
				log_fatal("Failed to save image to path.");
			}
		} else {
			Err(jdx::Error::CorruptFile)?
		}
	}

	Ok(())
}
