use std::io;
mod stegano;
mod utils;

fn main() {
    action_choice();
}

fn action_choice() {
    let mut choice = String::new();
    println!("What do you want to do ?\n1. Encrypt a message\n2. Decrypt a message");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

    match choice {
        1 => encrypt_message(),
        2 => decrypt_message(),
        _ => {
            println!("Invalid choice");
            action_choice();
        }
    }
}

fn encrypt_message() {
    let mut message = String::new();
    println!("Enter the message to encrypt:");

    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    encrypt_message_image(&message);
}

fn encrypt_message_image(message: &str) {
    let image_path = utils::get_random_image_path("images/");
    let output_path = "output.png";

    if let Err(e) = stegano::image::hide_message_in_image(&image_path, output_path, &message.trim()) {
        println!("Error hiding message: {}", e);
    } else {
        println!("Message hidden successfully in '{}'", output_path);
    }
}

fn decrypt_message() {
    decrypt_message_image();
}

fn decrypt_message_image() {
    let mut image_path = String::new();
    println!("Et1ner the path to the image:");

    io::stdin()
        .read_line(&mut image_path)
        .expect("Failed to read line");

    match stegano::image::extract_message_from_image(&image_path.trim()) {
        Ok(message) => println!("Message: \n{}", message),
        Err(e) => println!("Error extracting message: {}", e),
    }
}
