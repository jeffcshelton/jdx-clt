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

	let header = dataset.header();

	for (i, (image, label_index)) in dataset.iter().enumerate() {
		let label_path = output_path.join(&header.labels[usize::from(*label_index)]);
		let image_path = label_path.join(i.to_string() + ".png");

		if !label_path.exists() {
			fs::create_dir(&label_path)?;
		}

		if header.bit_depth == 8 {
			let image = GrayImage::from_raw(
				header.image_width.into(),
				header.image_height.into(),
				image.to_vec()
			).unwrap();

			if image.save_with_format(&image_path, ImageFormat::Png).is_err() {
				log_fatal("Failed to save image to path.");
			}
		} else if header.bit_depth == 24 {
			let image = RgbImage::from_raw(
				header.image_width.into(),
				header.image_height.into(),
				image.to_vec()
			).unwrap();

			if image.save_with_format(&image_path, ImageFormat::Png).is_err() {
				log_fatal("Failed to save image to path.");
			}
		} else if header.bit_depth == 32 {
			let image = RgbaImage::from_raw(
				header.image_width.into(),
				header.image_height.into(),
				image.to_vec()
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
