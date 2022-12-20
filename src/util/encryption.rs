use aes_gcm::{
    aead::{AeadInPlace, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

pub fn encrypt_with_random_key(mut buffer: Vec<u8>) -> (Key<Aes256Gcm>, Vec<u8>) {
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    cipher
        .encrypt_in_place_detached(nonce, b"imageshelter-v2", &mut buffer)
        .unwrap();

    (key, buffer)
}
