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
