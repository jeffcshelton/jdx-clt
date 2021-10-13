use jdx_rust::jdx::{self, Dataset};

pub fn concatenate(inputs: Vec<String>, output: String) -> jdx::Result<()> {
    let mut concat_dset = Dataset::read_from_path(&inputs[0])?;

    for i in 1..inputs.len() {
        let ref path = inputs[i];
        let input_dset = Dataset::read_from_path(path)?;

        concat_dset.append(&input_dset)?;
    }

    concat_dset.header.version = jdx::Version::current();
    concat_dset.write_to_path(output)?;

    Ok(())
}
