use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Failed to decrypt data")]
    DecryptionError,
    #[error("Failed to convert decrypted data to UTF-8")]
    Utf8ConversionError,
}

pub fn encrypt(key: String, plaintext: String) -> Result<Vec<u8>, CryptoError> {
    let key = resize_key(key, 32);
    let key = Key::<Aes256Gcm>::from_slice(key.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);
    let ciphered_data = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|_| CryptoError::DecryptionError)?;
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);
    Ok(encrypted_data)
}

pub fn decrypt(key: String, encrypted_data: Vec<u8>) -> Result<String, CryptoError> {
    let key = resize_key(key, 32);
    let key = Key::<Aes256Gcm>::from_slice(key.as_bytes());
    if encrypted_data.len() < 12 {
        return Err(CryptoError::DecryptionError);
    }
    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);
    let cipher = Aes256Gcm::new(key);
    let plaintext = cipher
        .decrypt(nonce, ciphered_data)
        .map_err(|_| CryptoError::DecryptionError)?;
    String::from_utf8(plaintext).map_err(|_| CryptoError::Utf8ConversionError)
}

pub fn resize_key(key: String, length: usize) -> String {
    let mut resized_key = key;
    if resized_key.len() < length {
        resized_key.push_str(&"0".repeat(length - resized_key.len()));
    } else if resized_key.len() > length {
        resized_key.truncate(length);
    }
    resized_key
}
