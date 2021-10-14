use std::{ffi::OsStr, fs, path::PathBuf};
use crate::log::{log_fatal, log_warning};
use jdx_rust::jdx::{self, Dataset};
use image::{ColorType, GenericImageView, io::Reader as ImageReader};

fn string_from_color(color: ColorType) -> &'static str {
    match color {
        ColorType::L8 => "L8",
        ColorType::La8 => "LA8",
        ColorType::Rgb8 => "RGB8",
        ColorType::Rgba8 => "RGBA8",
        ColorType::L16 => "L16",
        ColorType::La16 => "LA16",
        ColorType::Rgb16 => "RGB16",
        ColorType::Rgba16 => "RGBA16",
        ColorType::Bgr8 => "BGR8",
        ColorType::Bgra8 => "BGRA8",
        _ => "UNKNOWN",
    }
}

// TODO: Consider handling fatal errors with the result instead
pub fn generate(input: String, output: String) -> jdx::Result<()> {
    let input_path = PathBuf::from(input);
    let output_path = PathBuf::from(&output);

    let category_iterator = fs::read_dir(input_path)
        .unwrap_or_else(|_| log_fatal("Invalid input path: Must be a valid directory."));
   
    if output_path.exists() {
        log_fatal("Invalid output path: File already exists.");
    }

    if output_path.extension() != Some(OsStr::new("jdx")) {
        log_warning("JDX files should end with extension '.jdx'.");
    }

    let mut dataset = Dataset::default();
    let mut expected_index: u64 = 0;

    for category in category_iterator {
        let category_path = category
            .unwrap_or_else(|_| log_fatal("Invalid input path: Cannot read directory."))
            .path();

        let category_string = category_path
            .file_name()
            .unwrap_or(OsStr::new("none"))
            .to_string_lossy();
        
        if category_string.starts_with('.') {
            continue;
        }

        let category_index = match category_string.parse() {
            Ok(index) => index,
            Err(_) => log_fatal(format!("Found directory '{}' but expected '{}'. Please ensure directories are labeled consecutively starting with '0'.", category_string, expected_index)),
        };

        expected_index += 1;

        let entry_iterator = match fs::read_dir(&category_path) {
            Ok(iterator) => iterator,
            Err(_) => {
                log_warning(format!("Skipping file '{}': Cannot iterate over directory.", category_string));
                continue;
            }
        };

        for entry in entry_iterator {
            let entry_path = entry
                .unwrap_or_else(|_| log_fatal(format!("Failed to iterate over files in directory '{}'.", category_string)))
                .path();
            
            let entry_string = entry_path
                .file_name()
                .unwrap_or(OsStr::new("none"))
                .to_string_lossy();

            if entry_string.starts_with('.') {
                continue;
            }
            
            let image = ImageReader::open(&entry_path)
                .unwrap_or_else(|_| log_fatal(format!("Cannot read file '{}' as an image.", entry_string)))
                .decode()
                .unwrap_or_else(|_| log_fatal(format!("Cannot decode file '{}' as an image.", entry_string)));
            
            // TODO: Consider adding size checks for image width and height
            let width = image.width() as u16;
            let height = image.height() as u16;

            let bit_depth = match image.color() {
                ColorType::L8 => 8,
                ColorType::Rgb8 => 24,
                ColorType::Rgba8 => 32,
                color => log_fatal(format!("Image at '{}' has unsupported color type '{}'.", entry_string, string_from_color(color))),
            };
            
            if dataset.header.item_count == 0 {
                dataset.header.image_width = width;
                dataset.header.image_height = height;
                dataset.header.bit_depth = bit_depth;
            }

            if width != dataset.header.image_width {
                return Err(jdx::Error::UnequalWidths)
            } else if height != dataset.header.image_height {
                return Err(jdx::Error::UnequalHeights)
            } else if bit_depth != dataset.header.bit_depth {
                return Err(jdx::Error::UnequalBitDepths)
            }

            dataset.items.push(jdx::Item {
                data: image.as_bytes().to_vec(),
                width: width,
                height: height,
                bit_depth: bit_depth,
                label: category_index,
            });
        }
    }

    dataset.header.item_count = dataset.items.len() as u64;
    dataset.header.version = jdx::Version::current();
    dataset.write_to_path(output)?;

    Ok(())
}
