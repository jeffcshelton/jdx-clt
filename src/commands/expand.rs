use image::{GrayImage, ImageFormat, RgbImage, RgbaImage};
use jdx_rust::jdx::{self, Dataset};
use std::{path::PathBuf, fs};
use crate::log::log_fatal;

pub fn expand(input: String, output: String) -> jdx::Result<()> {
	let dataset = Dataset::read_from_path(input)?;
	let output_path = PathBuf::from(&output);

	if output_path.exists() {
		log_fatal("Invalid output path: File/directory already exists.");
	}

	if fs::create_dir(&output_path).is_err() {
		log_fatal(format!("Failed to create directory at '{}'.", output));
	}

	for (i, item) in dataset.items.iter().enumerate() {
		let label_path = output_path.join(item.label.to_string());

		if !label_path.exists() && fs::create_dir(&label_path).is_err() {
			log_fatal(format!("Failed to create directory '{}'.", label_path.to_string_lossy()));
		}

		let image_path = label_path.join(format!("{}.png", i));

		// TODO: Try to optimize and simplify this code
		if dataset.header.bit_depth == 8 {
			let image = match GrayImage::from_vec(item.width as u32, item.height as u32, item.data.clone()) {
				Some(image) => image,
				None => log_fatal(format!("Failed to parse data into image for path '{}'.", image_path.to_string_lossy())),
			};

			if image.save_with_format(&image_path, ImageFormat::Png).is_err() {
				log_fatal(format!("Failed to save image to path '{}'.", image_path.to_string_lossy()));
			}
		} else if dataset.header.bit_depth == 24 {
			let image = match RgbImage::from_vec(item.width as u32, item.height as u32, item.data.clone()) {
				Some(image) => image,
				None => log_fatal(format!("Failed to parse data into image for path '{}'.", image_path.to_string_lossy())),
			};

			if image.save_with_format(&image_path, ImageFormat::Png).is_err() {
				log_fatal(format!("Failed to save image to path '{}'.", image_path.to_string_lossy()));
			}
		} else if dataset.header.bit_depth == 32 {
			let image = match RgbaImage::from_vec(item.width as u32, item.height as u32, item.data.clone()) {
				Some(image) => image,
				None => log_fatal(format!("Failed to parse data into image for path '{}'.", image_path.to_string_lossy())),
			};

			if image.save_with_format(&image_path, ImageFormat::Png).is_err() {
				log_fatal(format!("Failed to save image to path '{}'.", image_path.to_string_lossy()));
			}
		} else {
			log_fatal(format!("Impossible bit depth '{}'. Please file an issue on GitHub for this error.", dataset.header.bit_depth));
		}
	}

	Ok(())
}
