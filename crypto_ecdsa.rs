//! ECDSA非对称加密 - 地址生成、签名、验签
use ring::signature::{ECDSA_P256_SHA256_ASN1, KeyPair, UnparsedPublicKey};
use ring::rand::SystemRandom;
use hex;

pub struct ECDSAHandler {
    rand: SystemRandom,
}

impl ECDSAHandler {
    pub fn new() -> Self {
        Self {
            rand: SystemRandom::new(),
        }
    }

    pub fn generate_key_pair(&self) -> Vec<u8> {
        let pk = KeyPair::generate(&ECDSA_P256_SHA256_ASN1, &self.rand).unwrap();
        pk.to_vec()
    }

    pub fn get_public_key(&self, key_pair: &[u8]) -> Vec<u8> {
        let kp = KeyPair::from_private_key(&ECDSA_P256_SHA256_ASN1, key_pair).unwrap();
        kp.public_key().as_ref().to_vec()
    }

    pub fn sign_message(&self, key_pair: &[u8], msg: &[u8]) -> Vec<u8> {
        let kp = KeyPair::from_private_key(&ECDSA_P256_SHA256_ASN1, key_pair).unwrap();
        kp.sign(&msg).as_ref().to_vec()
    }

    pub fn verify_signature(&self, pub_key: &[u8], msg: &[u8], sig: &[u8]) -> bool {
        let public_key = UnparsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, pub_key);
        public_key.verify(msg, sig).is_ok()
    }

    pub fn pub_key_to_address(&self, pub_key: &[u8]) -> String {
        let hash = ring::digest::digest(&ring::digest::SHA256, pub_key);
        format!("0x{}", hex::encode(&hash.as_ref()[0..20]))
    }
}
