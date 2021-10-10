use jdx_rust::jdx::{
    Header,
    ColorType,
};

use crate::log::log_error;

pub fn summarize(inputs: Vec<String>) {
    for path in inputs {
        println!();

        if !path.ends_with(".jdx") {
            log_error(format!("File '{}' is not a JDX file.", path));
            continue;
        }

        let header = Header::from_file(&path);
        if header.is_err() {
            log_error(format!("Failed to open file '{}'. File could be corrupted, not exist, or require permissions.", path));
            continue;
        }

        let header = header.unwrap();
        let color_text = match header.color_type {
            ColorType::Gray => "\x1b[38;5;8;1mGRAY\x1b[0m",
            ColorType::RGB => "\x1b[1;31mR\x1b[32mG\x1b[34mB\x1b[0m",
            ColorType::RGBA => "\x1b[1;31R\x1b[32mG\x1b[34mB\x1b[37mA\x1b[0m",
        };

        println!("=== \x1b[33;1m{}\x1b[0m ===", path);
        println!("JDX File \x1b[34mv{}.{}.{}\x1b[0m", header.version.major, header.version.minor, header.version.patch);
        println!("Color type: {}", color_text);
        println!("Image size: \x1b[1m{}\x1b[0m x \x1b[1m{}\x1b[0m", header.image_width, header.image_height);
        println!("Number of images: \x1b[1m{}\x1b[0m", header.item_count);
    }

    println!();
}
