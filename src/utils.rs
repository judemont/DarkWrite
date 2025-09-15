use std::{fs};

use rand::seq::IndexedRandom;

pub fn get_random_image_path(directory: &str) -> String {
    let paths = fs::read_dir(directory).unwrap();
    let mut images = Vec::new();

    for path in paths {
        if let Ok(entry) = path {
            if entry.path().is_file() {
                images.push(entry.path());
            }
        }
    }

    if images.is_empty() {
        panic!("No images found in the directory '{}'", directory);
    }

    let mut rng = rand::rng();
    return images
        .choose(&mut rng)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
}


pub fn is_valid_char(c: &char) -> bool {
    c.is_ascii() || c.is_alphabetic() || c.is_whitespace()
}