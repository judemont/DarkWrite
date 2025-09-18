use std::{fs};

use rand::seq::IndexedRandom;

pub fn get_random_image_path(directory: &str) -> Result<String, std::io::Error> {
    let paths = fs::read_dir(directory)?;
    let mut images = Vec::new();

    for path in paths {
        if let Ok(entry) = path {
            if entry.path().is_file() {
                images.push(entry.path());
            }
        }
    }

    if images.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("No images found in the directory '{}'", directory),
        ));
    }

    let mut rng = rand::rng();
    return Ok(images
        .choose(&mut rng)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string());
}

