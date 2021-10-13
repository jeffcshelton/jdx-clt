use jdx_rust::jdx::{self, Header};
use crate::log::log_warning;

pub fn summarize(inputs: Vec<String>) -> jdx::Result<()> {
    for path in inputs {
        println!();

        let header = Header::read_from_path(&path)?;
        let color_text = match header.bit_depth {
            8 => "\x1b[38;5;8;1mGRAY\x1b[0m",
            24 => "\x1b[1;31mR\x1b[32mG\x1b[34mB\x1b[0m",
            32 => "\x1b[1;31R\x1b[32mG\x1b[34mB\x1b[37mA\x1b[0m",
            _ => return Err(jdx::Error::CorruptFile(path)),
        };

        println!("=== \x1b[33;1m{}\x1b[0m ===", path);
        println!("JDX File \x1b[34mv{}.{}.{}\x1b[0m", header.version.major, header.version.minor, header.version.patch);
        println!("Color type: {} ({} bits)", color_text, header.bit_depth);
        println!("Image size: \x1b[1m{}\x1b[0m x \x1b[1m{}\x1b[0m", header.image_width, header.image_height);
        println!("Number of images: \x1b[1m{}\x1b[0m", header.item_count);

        if !path.ends_with(".jdx") {
            log_warning(format!("JDX files should end with extension '.jdx'."));
        }
    }

    println!();
    Ok(())
}
