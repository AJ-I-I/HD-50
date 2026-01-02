use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};
use rand::RngCore;

// Constants
const SALT_LEN: usize = 22; // Argon2 salt string length is variable but we store the string
// Format: [Salt (16 bytes raw)][Nonce (12 bytes)][Ciphertext]
// derive a 32-byte key.

pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    let _ = argon2::Argon2::default().hash_password_into(
        password.as_bytes(), 
        salt, 
        &mut key
    );

    let params = argon2::Params::default();
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    key
}

pub struct CryptoContext {
    cipher: Aes256Gcm,
}

impl CryptoContext {
    pub fn new(key: &[u8; 32]) -> Self {
        Self {
            cipher: Aes256Gcm::new(key.into()),
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> (Vec<u8>, [u8; 12]) {
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        let crypto_nonce = Nonce::from_slice(&nonce);
        
        let ciphertext = self.cipher.encrypt(crypto_nonce, data).expect("Encryption failure");
        (ciphertext, nonce)
    }

    pub fn decrypt(&self, data: &[u8], nonce: &[u8; 12]) -> Result<Vec<u8>, aes_gcm::aead::Error> {
        let crypto_nonce = Nonce::from_slice(nonce);
        self.cipher.decrypt(crypto_nonce, data)
    }
}
