use aes_gcm::{
    aead::{KeyInit, Aead},
    Aes256Gcm, Nonce,
};
use rand::{Rng, SeedableRng};
use sha2::Digest;

pub fn encrypt_with_random_key(buffer: Vec<u8>) -> (String, Vec<u8>) {
    let mut raw_key = [0u8; 32];
    let mut nonce = [0u8; 12];
    
    rand::rngs::StdRng::from_entropy().fill(&mut raw_key);
    rand::rngs::StdRng::from_entropy().fill(&mut nonce);

    let key = Aes256Gcm::new_from_slice(raw_key.as_ref()).unwrap();
    let nonce = Nonce::from_slice(nonce.as_ref());

    let output = key
        .encrypt(nonce, buffer.as_ref())
        .unwrap();

    let mut key_with_nonce = nonce.to_vec();
    key_with_nonce.append(&mut raw_key.to_vec());

    (bs58::encode(&key_with_nonce).into_string(), output)
}

pub fn decrypt_with_key(buffer: Vec<u8>, key: String) -> Result<Vec<u8>, String> {
    let key_with_nonce = match bs58::decode(key).into_vec() {
        Ok(key) => key,
        Err(_) => return Err("Failed to decode the key".to_string())
    };
    let nonce = Nonce::from_slice(&key_with_nonce[0..12]);
    let mut key: Option<Aes256Gcm> = None;

    // DEPRECATED, INSECURE, UNSAFE, DO NOT USE, ONLY FOR BACKWARD COMPATIBILITY
    if key_with_nonce.len() == 16 {
        let raw_key = i32::from_be_bytes(match key_with_nonce[12..16].try_into() {
            Ok(key) => key,
            Err(_) => return Err("Failed to convert the last 4 bytes to integer".to_string())
        });
        let sha_key = sha2::Sha256::digest(&raw_key.to_be_bytes()).to_vec();
        key = match Aes256Gcm::new_from_slice(sha_key.as_ref()) {
            Ok(key) => key.into(),
            Err(_) => return Err("Failed to create key".to_string())
        };
    }

    if key_with_nonce.len() == 44 {
        key = match Aes256Gcm::new_from_slice(&key_with_nonce[12..44]) {
            Ok(key) => key.into(),
            Err(_) => return Err("Failed to create key".to_string())
        };
    }

    if key.is_none() {
        return Err("Failed to create key".to_string());
    }

    Ok(match key.unwrap().decrypt(nonce, buffer.as_ref()) {
        Ok(output) => output,
        Err(_) => return Err("Failed to decrypt the file".to_string())
    })
}