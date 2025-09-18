use image::{DynamicImage, ImageResult};
use rand::random;

pub fn hide_message_in_image(
    image_path: &str,
    output_path: &str,
    message: Option<&str>,
    binary_message: Option<&[u8]>,
) -> ImageResult<()> {
    let img = image::open(image_path)?;
    let mut pixels = img.to_rgba8();

    let binary_message = match (message, binary_message) {
        (Some(msg), None) => msg.as_bytes(),
        (None, Some(b)) => b,
        _ => {
            return Err(image::ImageError::Unsupported(
                image::error::UnsupportedError::from_format_and_kind(
                    image::ImageFormat::Png.into(),
                    image::error::UnsupportedErrorKind::GenericFeature(
                        "Either message or binary_message must be provided.".to_string(),
                    ),
                ),
            ));
        }
    };

    let msg_len = binary_message.len() as u32;
    let mut bit_index: usize = 0;
    let total_bits = 32 + binary_message.len() * 8; 

    for pixel in pixels.chunks_mut(4) {
        if bit_index >= total_bits {
            break;
        }

        for color_byte in &mut pixel[0..3] {
            if bit_index >= total_bits {
                break;
            }

            let message_bit = if bit_index < 32 {
                ((msg_len >> (31 - bit_index)) & 1) as u8
            } else {
                let msg_bit_index = bit_index - 32;
                let message_byte_index = msg_bit_index / 8;
                let message_bit_index = msg_bit_index % 8;
                (binary_message[message_byte_index] >> (7 - message_bit_index)) & 1
            };
            let message_bit = message_bit ^ 1;

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

pub fn extract_data_from_image(image_path: &str) -> ImageResult<Vec<u8>> {
    let img = image::open(image_path)?;
    let pixels = img.to_rgba8();

    let mut msg_len: u32 = 0;
    let mut bit_index: usize = 0;


    let mut length_bits: Vec<u8> = Vec::with_capacity(32);
    let mut message_bits: Vec<u8> = Vec::new();

    for pixel in pixels.chunks(4) {
        for color_byte in &pixel[0..3] {
            let lsb = color_byte & 1;
            let lsb = lsb ^ 1; // Invert the bit to match encoding
            if bit_index < 32 {
                length_bits.push(lsb);
            } else {
                message_bits.push(lsb);
            }
            bit_index += 1;
            if bit_index >= 32 + message_bits.len() {
            }
        }
    }

    for i in 0..32 {
        msg_len <<= 1;
        msg_len |= length_bits[i] as u32;
    }

    let mut message_bytes: Vec<u8> = Vec::with_capacity(msg_len as usize);
    let mut cur_byte: u8 = 0;
    let mut cur_bit: usize = 0;
    for i in 0..(msg_len * 8) as usize {
        let bit = message_bits.get(i).copied().unwrap_or(0);
        cur_byte = (cur_byte << 1) | bit;
        cur_bit += 1;
        if cur_bit == 8 {
            message_bytes.push(cur_byte);
            cur_byte = 0;
            cur_bit = 0;
        }
    }

    Ok(message_bytes)
}
