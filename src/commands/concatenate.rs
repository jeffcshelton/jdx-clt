use crate::log_warning;
use jdx::Dataset;

pub fn concatenate(inputs: Vec<String>, output: String) -> jdx::Result<()> {
	if !output.ends_with(".jdx") {
		log_warning("JDX files should end with extension '.jdx'.");
	}

	let mut base_dataset = Dataset::read_from_path(inputs.get(0).unwrap())?;

	for path in &inputs[1..] {
		let dataset = Dataset::read_from_path(path)?;
		base_dataset.append(dataset)?;
	}

	// TODO: Add update to Dataset header version

	base_dataset.write_to_path(output)?;

	Ok(())
}
