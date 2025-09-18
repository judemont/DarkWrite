# DarkWrite

---

### Steganography + Encryption = DarkWrite
### Secure communication through invisible data
### 👻 Undetectable  .  🔒 Unbreakable  .  🚀 Unstoppable

---
**Trust no messenger. Assume no security. Everything is monitored. The only way out? Protect yourself. Encryption isn’t enough : *Makes your messages disappear into the darkness*.**

DarkWrite is an open-source tool for **secure steganography and encryption**, allowing you to hide encrypted messages inside images. Your secrets stay invisible until you decide to reveal them. 

![](readme_assets/dark.png)
*Image from [dark.fi](https://dark.fi), under [Viral Public License](https://viralpubliclicense.org)*

> [!WARNING]
> This project is still in beta and have not received external security review and may contain vulnerabilities. Do not use for sensitive use cases.


## Features ✨
- **Steganography**: Hide messages inside images by modifying pixel data.
- **Encryption**: Encrypt messages before hiding for extra security.
- **Simple CLI**: Interactive command-line interface for hiding and extracting messages.
- **End-to-End encryption [WIP]**: Planned support for secure key exchange and message confidentiality between sender and receiver.

## Security 🔐
- **Steganography with obfuscation**: DarkWrite uses LSB combined with obfuscation to help evade detection by steganalysis tools¹.
- **Military-Grade Encryption**: Messages are encrypted using **AES-256-GCM**, one of the most secure encryption algorithms available today.

> ¹ DarkWrite's steganography has not been detected by any of the steganography detection tools I have had the opportunity to try (with or without encryption).

## Usage 🚀

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
4. Enter image file path.
5. The message is extracted and displayed.

---

> [!NOTE]
> **Not only** for educational purposes, but for all ethical use of your free speech, everywhere in the cyber-space.