use image::{DynamicImage, ImageResult};
use rand::random;

pub fn hide_message_in_image(
    image_path: &str,
    output_path: &str,
    message: &str,
) -> ImageResult<()> {

    let img = image::open(image_path)?;
    let mut pixels = img.to_rgba8();
    let binary_message = message.as_bytes();

    let total_bits = binary_message.len() * 8;
    let mut bit_index: usize = 0; 

    for pixel in pixels.chunks_mut(4) {
        if bit_index >= total_bits {
            break;
        }

        for color_byte in &mut pixel[0..3] {
            if bit_index >= total_bits {
                break;
            }

            let message_byte_index = bit_index / 8;
            let message_bit_index = bit_index % 8;

            let message_bit = (binary_message[message_byte_index] >> (7 - message_bit_index)) & 1;
            let message_bit = message_bit ^ 1; // Invert the bit to make it harder to detect the steganography.

            let color_lsb = *color_byte & 1;

            if color_lsb != message_bit {
                if *color_byte == 0 {
                    *color_byte += 1;
                } else if *color_byte == 255 {
                    *color_byte -= 1;
                } else {
                    if random() {
                        *color_byte += 1;
                    } else {
                        *color_byte -= 1;
                    }
                }
            }

            bit_index += 1;
        }
    }

    let hidden_img = DynamicImage::ImageRgba8(pixels);
    hidden_img.save(output_path)?;

    Ok(())
}


pub fn extract_message_from_image(image_path: &str) -> ImageResult<String> {
    let img = image::open(image_path)?;
    let pixels = img.to_rgba8();

    let mut binary_message: Vec<u8> = Vec::new();
    let mut current_byte: u8 = 0;
    let mut bit_count: usize = 0;

    for pixel in pixels.chunks(4) {
        for color_byte in &pixel[0..3] {
            let lsb = color_byte & 1;
            current_byte = (current_byte << 1) | lsb;
            current_byte = current_byte ^ 1; // Invert the bit to match the encoding process.
            bit_count += 1;

            if bit_count == 8 {
                binary_message.push(current_byte);
                current_byte = 0;
                bit_count = 0;
            }
        }
    }

    Ok(String::from_utf8_lossy(&binary_message).to_string())
}