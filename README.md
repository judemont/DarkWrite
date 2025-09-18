# DarkWrite

---

### Steganography + Encryption = DarkWrite
### Secure communication through invisible data
### 👻 Undetectable  .  🔒 Unbreakable  .  🚀 Unstoppable

---

## Overview
> Trust no messenger. Assume no security. Everything is monitored. The only way out? Protect yourself. Encryption isn’t enough : *Makes your messages disappear into the darkness*.

DarkWrite is an open-source tool for **secure steganography and encryption**, allowing you to hide encrypted messages inside images. Your secrets stay invisible until you decide to reveal them. 

## Features

- **Steganography**: Hide messages inside images by modifying pixel data.
- **Encryption**: Encrypt messages before hiding for extra security.
- **Simple CLI**: Interactive command-line interface for hiding and extracting messages.
- **End-to-End encryption [WIP]**: Planned support for secure key exchange and message confidentiality between sender and receiver.

## Security
- **Undetectable steganography**: DarkWrite uses advanced steganography techniques to be undetectable by steganalysis tools¹.
- **Secure encryption**: DarkWrite uses the AES-256 encryption algorithm, known to be one of the most secure to date.

> ¹ DarkWrite's steganography has not been detected by any of the steganography detection tools I have had the opportunity to try (with or without encryption).

## Usage

### Run the program
```bash
cargo run --release
```

### Hiding a Message

1. The message will be hidden in a randomly chosen image from the `images/` folder. The `images/` folder contains example images by default. For better undetectability, avoid using these example images.
2. Choose "Hide a message".
3. Select encryption method:
   - No encryption: Message can be extracted by anyone using DarkWrite.
   - AES-256: Enter a key to encrypt your message before hiding.
4. Enter your message.
5. The message is hidden in a randomly chosen image from the `images/` folder and saved as `output.png`.

### Extracting a Message

1. Run the program:
   ```bash
   cargo run --release
   ```
2. Choose "Extract a message".
3. Select decryption method:
   - No key: Extracts without key or password.
   - AES-256: Enter the key to decrypt the extracted message.
4. The message is extracted from `output.png` and displayed.

## How It Works

- The message length is encoded in the first 32 bits of the image.
- Each bit of the message is hidden in the least significant bit of the image’s pixel data.
- For encryption, AES-256-GCM is used to secure the message before embedding.
- Extraction reads the length, then reconstructs the message from the image data, and decrypts if needed.

