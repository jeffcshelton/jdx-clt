use std::{path::PathBuf, ffi::OsStr};
use crate::log_warning;
use jdx::Dataset;

pub fn concatenate(input_paths: Vec<PathBuf>, output_path: PathBuf) -> jdx::Result<()> {
	if output_path.extension().and_then(OsStr::to_str) != Some("jdx") {
		log_warning("JDX files should end with extension '.jdx'.");
	}

	let mut base_dataset = Dataset::read_from_path(input_paths.get(0).unwrap())?;

	for path in &input_paths[1..] {
		let dataset = Dataset::read_from_path(path)?;
		base_dataset.append(dataset)?;
	}

	// TODO: Add update to Dataset header version

	base_dataset.write_to_path(output_path)?;

	Ok(())
}
