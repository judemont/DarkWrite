use aes_gcm::{
    aead::{ Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Key, Nonce
};

// Based on https://vivekshuk.la/tech/aes-encryption-rust
// ---
pub fn encrypt(key: String, plaintext: String) -> Vec<u8> {
    let key = resize_key(key, 32);
    let key = Key::<Aes256Gcm>::from_slice(key.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(key);

    let ciphered_data = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .expect("failed to encrypt");

    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);

    encrypted_data
}

pub fn decrypt(key: String, encrypted_data: Vec<u8>) -> String {
    let key = resize_key(key, 32);
    let key = Key::<Aes256Gcm>::from_slice(key.as_bytes());

    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);

    let cipher = Aes256Gcm::new(key);

    let plaintext = cipher
        .decrypt(nonce, ciphered_data)
        .expect("failed to decrypt data");

    String::from_utf8(plaintext).expect("failed to convert vector of bytes to string")
}
// ---

// Because the key must be exactly 32 bytes long
pub fn resize_key(key: String, length: i16 ) -> String {
    let mut resized_key = key;
    while resized_key.len() < length as usize {
        resized_key.push('0');
    }
    resized_key.truncate(length as usize);
    resized_key
}
