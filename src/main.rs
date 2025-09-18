use std::io;
mod crypto;
mod stegano;
mod utils;

fn main() {
    println!("Welcome to DarkWrite");
    println!("===================================================");
    println!("Encrypts messages that no one even knows exist.");
    println!("===================================================");
    action_choice(0);
}

fn action_choice(options_state: u8) {
    let mut choice = String::new();

    match options_state {
        0 => {
            // Initial state
            println!("\nWhat do you want to do ?\n1. Hide a message\n2. Extract a message");
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");
            let choice: u32 = choice.trim().parse().expect("Please enter a valid number");
            match choice {
                1 => {
                    action_choice(1);
                }
                2 => {
                    action_choice(2);
                }
                _ => {
                    println!("Invalid choice");
                    action_choice(0);
                }
            }
        }
        1 => {
            // Hide
            println!(
                "\nWhich encryption method do you want to use? \n1. No encryption\n2. Encrypt with a key (AES-256)"
            );
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");
            let choice: u32 = choice.trim().parse().expect("Please enter a valid number");
            match choice {
                1 => {
                    action_choice(3);
                }
                2 => {
                    action_choice(4);
                }
                _ => {
                    println!("Invalid choice");
                    action_choice(1);
                }
            }
        }

        2 => {
            // Extract
            println!(
                "\nWhich decryption method do you want to use? \n1. No decryption key\n2. Decrypt with a key (AES-256)"
            );
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");
            let choice: u32 = choice.trim().parse().expect("Please enter a valid number");
            match choice {
                1 => {
                    action_choice(5);
                }
                2 => {
                    action_choice(6);
                }
                _ => {
                    println!("Invalid choice");
                    action_choice(1);
                }
            }
        }
        3 => {
            // No encryption, hide
            let mut message = String::new();
            println!("\nEnter the message to hide:");

            io::stdin()
                .read_line(&mut message)
                .expect("Failed to read line");

            let image_path = utils::get_random_image_path("images/");
            let output_path = "output.png";

            if let Err(e) = stegano::image::hide_message_in_image(
                &image_path,
                output_path,
                Some(&message.trim()),
                None,
            ) {
                println!("Error hiding message: {}", e);
            } else {
                println!("Message hidden successfully in '{}'", output_path);
            }
        }
        4 => {
            // Encrypt with AES-256
            let mut message = String::new();
            println!("\nEnter the message to hide:");
            io::stdin()
                .read_line(&mut message)
                .expect("Failed to read line");

            let mut key = String::new();
            println!("\nEnter the encryption key:");
            io::stdin()
                .read_line(&mut key)
                .expect("Failed to read line");

            match crypto::aes::encrypt(key.trim().to_string(), message.trim().to_string()) {
                Ok(encrypted_message) => {
                    let image_path = utils::get_random_image_path("images/");

                    let output_path = "output.png";
                    
                    if let Err(e) = stegano::image::hide_message_in_image(
                        &image_path,
                        output_path,
                        None,
                        Some(&encrypted_message),
                    ) {
                        println!("Error hiding message: {}", e);
                    } else {
                        println!("Message hidden successfully in '{}'", output_path);
                    }
                }
                Err(e) => {
                    println!("Error encrypting message: {}", e);
                }
            }
        }

        5 => {
            // No decryption key, extract
            let mut image_path = String::new();
            println!("\nEnter the path to the image:");

            io::stdin()
                .read_line(&mut image_path)
                .expect("Failed to read line");

            match stegano::image::extract_message_from_image(&image_path.trim()) {
                Ok(message) => {
                    let message = filter_message(&message);
                    show_extracted_message(&message);
                }
                Err(e) => println!("Error extracting message: {}", e),
            }
        }
        6 => {
            // Decrypt with AES-256, extract
            let mut image_path = String::new();
            println!("\nEnter the path to the image:");
            io::stdin()
                .read_line(&mut image_path)
                .expect("Failed to read line");

            let mut key = String::new();
            println!("\nEnter the encryption key:");
            io::stdin()
                .read_line(&mut key)
                .expect("Failed to read line");

            match stegano::image::extract_message_from_image(&image_path.trim()) {
                Ok(message) => {
                    let message = filter_message(&message);
                    match crypto::aes::decrypt(key.trim().to_string(), message.into_bytes()) {
                        Ok(decrypted_message) => show_extracted_message(&decrypted_message),
                        Err(e) => println!("Error decrypting message: {}", e),
                    }
                }
                Err(e) => println!("Error extracting message: {}", e),
            }
        }
        _ => {
            println!("Invalid choice");
            action_choice(0);
        }
    }
}

fn show_extracted_message(message: &String) {
    println!("\nExtracted message:");
    println!("-----------------------------------");
    println!("{}", message);
    println!("\n-----------------------------------");
}

fn filter_message(message: &String) -> String {
    let mut filtered_message = String::new();
    for char in message.chars() {
        if utils::is_valid_char(&char) {
            filtered_message.push(char);
        }
    }
    filtered_message
}
