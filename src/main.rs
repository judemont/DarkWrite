use std::env;
use std::process;
mod crypto;
mod stegano;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    let command = &args[1];
    match command.as_str() {
        "hide" => handle_hide_command(&args),
        "hide-file" => handle_hide_file_command(&args),
        "extract" => handle_extract_command(&args),
        "--help" | "-h" => print_usage(),
        _ => {
            println!("Invalid command: {}", command);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("===================================================");
    println!("DarkWrite - Secure communication through invisible data.");
    println!("===================================================");
    println!("👻 Undetectable . 🔒 Unbreakable . 🚀 Unstoppable");
    println!("===================================================");
    println!();
    println!("USAGE:");
    println!("  darkwrite <COMMAND> [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("  hide        Hide a message in an image");
    println!("  hide-file   Hide a file in an image");
    println!("  extract     Extract a message or file from an image");
    println!();
    println!("HIDE MESSAGE OPTIONS:");
    println!("  darkwrite hide <MESSAGE> [--key <KEY>] [--output <OUTPUT_PATH>]");
    println!("    MESSAGE        The message to hide");
    println!("    --key, -k      AES-256 encryption key (optional)");
    println!("    --output, -o   Output path (default: output.png)");
    println!();
    println!("HIDE FILE OPTIONS:");
    println!("  darkwrite hide-file <FILE_PATH> [--key <KEY>] [--output <OUTPUT_PATH>]");
    println!("    FILE_PATH      Path to the file to hide");
    println!("    --key, -k      AES-256 encryption key (optional)");
    println!("    --output, -o   Output path (default: output.png)");
    println!();
    println!("EXTRACT OPTIONS:");
    println!("  darkwrite extract <IMAGE_PATH> [--key <KEY>] [--output <OUTPUT_PATH>]");
    println!("    IMAGE_PATH     Path to the image containing the message or file");
    println!("    --key, -k      AES-256 decryption key (if needed)");
    println!("    --output, -o   Output path to save extracted data (optional)");
    println!();
    println!("EXAMPLES:");
    println!("  darkwrite hide \"My secret message\"");
    println!("  darkwrite hide \"My message\" --key \"my_secret_key\" --output \"result.png\"");
    println!("  darkwrite hide-file \"secret.pdf\" --key \"my_secret_key\" --output \"result.png\"");
    println!("  darkwrite extract \"result.png\"");
    println!("  darkwrite extract \"result.png\" --key \"my_secret_key\"");
    println!("  darkwrite extract \"result.png\" --key \"my_secret_key\" --output \"file.pdf\"");
}

fn handle_hide_file_command(args: &[String]) {
    if args.len() < 3 {
        println!("Error: Missing file path");
        println!("Usage: darkwrite hide-file <FILE_PATH> [--key <KEY>] [--output <OUTPUT_PATH>]");
        process::exit(1);
    }
    let file_path = &args[2];
    let mut key: Option<String> = None;
    let mut output_path = "output.png".to_string();

    // Parse optional arguments
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--key" | "-k" => {
                if i + 1 < args.len() {
                    key = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    println!("Error: Missing value for --key");
                    process::exit(1);
                }
            }
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    output_path = args[i + 1].clone();
                    i += 2;
                } else {
                    println!("Error: Missing value for --output");
                    process::exit(1);
                }
            }
            _ => {
                println!("Unknown argument: {}", args[i]);
                process::exit(1);
            }
        }
    }

    // Read file contents
    let file_bytes = match std::fs::read(file_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("Error reading file: {}", e);
            process::exit(1);
        }
    };

    // Get a random image path
    let image_path = match utils::get_random_image_path("images/") {
        Ok(path) => path,
        Err(_) => {
            println!("Error: You must add images to the './images/' directory");
            process::exit(1);
        }
    };

    // Process based on whether a key is provided
    match key {
        Some(encryption_key) => {
            // Encrypt with AES-256
            match crypto::aes::encrypt(encryption_key, String::from_utf8_lossy(&file_bytes).to_string()) {
                Ok(encrypted_file) => {
                    if let Err(e) = stegano::image::hide_message_in_image(
                        &image_path,
                        &output_path,
                        None,
                        Some(&encrypted_file),
                    ) {
                        println!("Error hiding file: {}", e);
                        process::exit(1);
                    } else {
                        println!("File encrypted and hidden successfully in '{}'", output_path);
                    }
                }
                Err(e) => {
                    println!("Encryption error: {}", e);
                    process::exit(1);
                }
            }
        }
        None => {
            // No encryption
            if let Err(e) = stegano::image::hide_message_in_image(
                &image_path,
                &output_path,
                None,
                Some(&file_bytes),
            ) {
                println!("Error hiding file: {}", e);
                process::exit(1);
            } else {
                println!("File hidden successfully in '{}'", output_path);
            }
        }
    }
}

fn handle_hide_command(args: &[String]) {
    if args.len() < 3 {
        println!("Error: Missing message");
        println!("Usage: darkwrite hide <MESSAGE> [--key <KEY>] [--output <OUTPUT_PATH>]");
        process::exit(1);
    }
    let message = &args[2];
    let mut key: Option<String> = None;
    let mut output_path = "output.png".to_string();

    // Parse optional arguments
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--key" | "-k" => {
                if i + 1 < args.len() {
                    key = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    println!("Error: Missing value for --key");
                    process::exit(1);
                }
            }
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    output_path = args[i + 1].clone();
                    i += 2;
                } else {
                    println!("Error: Missing value for --output");
                    process::exit(1);
                }
            }
            _ => {
                println!("Unknown argument: {}", args[i]);
                process::exit(1);
            }
        }
    }

    // Get a random image path
    let image_path = match utils::get_random_image_path("images/") {
        Ok(path) => path,
        Err(_) => {
            println!("Error: You must add images to the './images/' directory");
            process::exit(1);
        }
    };

    // Process based on whether a key is provided
    match key {
        Some(encryption_key) => {
            // Encrypt with AES-256
            match crypto::aes::encrypt(encryption_key, message.clone()) {
                Ok(encrypted_message) => {
                    if let Err(e) = stegano::image::hide_message_in_image(
                        &image_path,
                        &output_path,
                        None,
                        Some(&encrypted_message),
                    ) {
                        println!("Error hiding message: {}", e);
                        process::exit(1);
                    } else {
                        println!(
                            "Message encrypted and hidden successfully in '{}'",
                            output_path
                        );
                    }
                }
                Err(e) => {
                    println!("Encryption error: {}", e);
                    process::exit(1);
                }
            }
        }
        None => {
            // No encryption
            if let Err(e) = stegano::image::hide_message_in_image(
                &image_path,
                &output_path,
                Some(message),
                None,
            ) {
                println!("Error hiding message: {}", e);
                process::exit(1);
            } else {
                println!("Message hidden successfully in '{}'", output_path);
            }
        }
    }
}

fn handle_extract_command(args: &[String]) {
    if args.len() < 3 {
        println!("Error: Missing image path");
        println!("Usage: darkwrite extract <IMAGE_PATH> [--key <KEY>]");
        process::exit(1);
    }
    let image_path = &args[2];
    let mut key: Option<String> = None;
    let mut output_path: Option<String> = None;

    // Parse optional arguments
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--key" | "-k" => {
                if i + 1 < args.len() {
                    key = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    println!("Error: Missing value for --key");
                    process::exit(1);
                }
            }
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    output_path = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    println!("Error: Missing value for --output");
                    process::exit(1);
                }
            }
            _ => {
                println!("Unknown argument: {}", args[i]);
                process::exit(1);
            }
        }
    }

    // Extract the message or file
    match stegano::image::extract_data_from_image(image_path) {
        Ok(extracted_data) => {
            match key {
                Some(decryption_key) => {
                    // Decrypt with AES-256
                    match crypto::aes::decrypt(decryption_key, extracted_data) {
                        Ok(decrypted_message) => {
                            if let Some(path) = output_path {
                                match std::fs::write(&path, decrypted_message.as_bytes()) {
                                    Ok(_) => println!("Decrypted data saved to '{}'", path),
                                    Err(e) => println!("Error saving to file: {}", e),
                                }
                            } else {
                                show_extracted_message(&decrypted_message);
                            }
                        }
                        Err(e) => {
                            println!("Decryption error: {}", e);
                            process::exit(1);
                        }
                    }
                }
                None => {
                    if let Some(path) = output_path {
                        match std::fs::write(&path, &extracted_data) {
                            Ok(_) => println!("Extracted data saved to '{}'", path),
                            Err(e) => println!("Error saving to file: {}", e),
                        }
                    } else {
                        let message = String::from_utf8_lossy(&extracted_data).to_string();
                        show_extracted_message(&message);
                    }
                }
            }
        }
        Err(e) => {
            println!("Extraction error: {}", e);
            process::exit(1);
        }
    }
}

fn show_extracted_message(message: &str) {
    println!("\nExtracted message:");
    println!("-----------------------------------");
    println!("{}", message);
    println!("-----------------------------------");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_aes_encryption() {
        let key = "my_secret_key_1234567890".to_string();
        let plaintext = "Hello, world! :3".to_string();
        // Encrypt the plaintext
        let encrypted = crypto::aes::encrypt(key.clone(), plaintext.clone()).unwrap();
        println!("Encrypted: {:?}", encrypted);
        // Decrypt the ciphertext
        let decrypted = crypto::aes::decrypt(key, encrypted).unwrap();
        println!("Decrypted: {:?}", decrypted);
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_stegano_hide_extract() {
        let message = "This is a secret message.".to_string();
        let image_path = match utils::get_random_image_path("images/") {
            Ok(path) => path,
            Err(_) => {
                println!("You should add images to the directory: 'images/'");
                return;
            }
        };
        let output_path = "test_output.png";
        stegano::image::hide_message_in_image(&image_path, output_path, Some(&message), None)
            .unwrap();
        let extracted_message = stegano::image::extract_data_from_image(output_path).unwrap();
        let extracted_message = String::from_utf8_lossy(&extracted_message).to_string();
        assert_eq!(message, extracted_message);
    }

    #[test]
    fn test_stegano_aes_hide_extract() {
        let message = "This is a secret message.".to_string();
        let image_path = match utils::get_random_image_path("images/") {
            Ok(path) => path,
            Err(_) => {
                println!("You should add images to the directory: 'images/'");
                return;
            }
        };
        let key = "my_secret_key_1234567890".to_string();
        let output_path = "test_output.png";
        // Encrypt the message
        let message_encrypted = crypto::aes::encrypt(key.clone(), message.clone()).unwrap();
        // Hide the message
        stegano::image::hide_message_in_image(
            &image_path,
            output_path,
            None,
            Some(&message_encrypted),
        )
        .unwrap();
        // Extract the message
        let extracted_message = stegano::image::extract_data_from_image(output_path).unwrap();
        println!("Extracted (encrypted) message: {:?}", extracted_message);
        // Decrypt the message
        let decrypted_message = crypto::aes::decrypt(key, extracted_message).unwrap();
        assert_eq!(message, decrypted_message);
    }
}
