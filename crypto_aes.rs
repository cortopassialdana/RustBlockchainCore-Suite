//! AES对称加密 - 链上敏感数据加密存储
use ring::aead::{AES_256_GCM, SealingKey, OpeningKey, Nonce, seal, open};
use ring::rand::SecureRandom;

pub struct AESCrypto {
    key: [u8; 32],
}

impl AESCrypto {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        let rng = ring::rand::SystemRandom::new();
        rng.fill(&mut key).unwrap();
        key
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let sealing_key = SealingKey::new(&AES_256_GCM, &self.key).unwrap();
        let mut nonce_bytes = [0u8; 12];
        ring::rand::SystemRandom::new().fill(&mut nonce_bytes).unwrap();
        let nonce = Nonce::try_assume_unique_for_key(&nonce_bytes).unwrap();
        
        let mut ciphertext = Vec::from(&nonce_bytes[..]);
        ciphertext.extend_from_slice(&seal(&sealing_key, nonce, data, b"").unwrap());
        ciphertext
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Option<Vec<u8>> {
        if ciphertext.len() < 12 {
            return None;
        }
        let (nonce_bytes, encrypted) = ciphertext.split_at(12);
        let opening_key = OpeningKey::new(&AES_256_GCM, &self.key).unwrap();
        let nonce = Nonce::try_assume_unique_for_key(nonce_bytes).unwrap();
        open(&opening_key, nonce, encrypted, b"").ok()
    }
}
