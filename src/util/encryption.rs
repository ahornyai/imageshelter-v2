use std::fmt::Error;

use aes_gcm::{
    aead::{KeyInit, Aead},
    Aes256Gcm, Nonce,
};
use sha2::Digest;

pub fn encrypt_with_random_key(buffer: Vec<u8>) -> (String, Vec<u8>) {
    let raw_key = rand::random::<i32>();
    let sha_key = sha2::Sha256::digest(&raw_key.to_be_bytes()).to_vec();
    let key = Aes256Gcm::new_from_slice(sha_key.as_ref()).unwrap();
    let nonce = rand::random::<[u8; 12]>();
    let nonce = Nonce::from_slice(nonce.as_ref());

    let output = key
        .encrypt(nonce, buffer.as_ref())
        .unwrap();

    let mut key_with_nonce = nonce.to_vec();
    key_with_nonce.append(&mut raw_key.to_be_bytes().to_vec());

    (bs58::encode(&key_with_nonce).into_string(), output)
}

pub fn decrypt_with_key(buffer: Vec<u8>, key: String) -> Result<Vec<u8>, String> {
    let key_with_nonce = match bs58::decode(key).into_vec() {
        Ok(key) => {
            if key.len() != 16 {
                return Err("Invalid key length".to_string());
            }
            key
        },
        Err(_) => return Err("Failed to decode the key".to_string())
    };
    let nonce = Nonce::from_slice(&key_with_nonce[0..12]);
    let raw_key = i32::from_be_bytes(match key_with_nonce[12..16].try_into() {
        Ok(key) => key,
        Err(_) => return Err("Failed to convert the last 4 bytes to integer".to_string())
    });
    let sha_key = sha2::Sha256::digest(&raw_key.to_be_bytes()).to_vec();
    let key = match Aes256Gcm::new_from_slice(sha_key.as_ref()) {
        Ok(key) => key,
        Err(_) => return Err("Failed to create key".to_string())
    };

    Ok(match key.decrypt(nonce, buffer.as_ref()) {
        Ok(output) => output,
        Err(_) => return Err("Failed to decrypt the file".to_string())
    })
}