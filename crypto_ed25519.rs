//! Ed25519椭圆曲线加密 - 高性能签名验签
use ring::signature::{Ed25519KeyPair, UnparsedPublicKey, ED25519};
use ring::rand::SystemRandom;

pub struct Ed25519Crypto {
    rand: SystemRandom,
}

impl Ed25519Crypto {
    pub fn new() -> Self {
        Self {
            rand: SystemRandom::new(),
        }
    }

    pub fn generate_keypair(&self) -> Vec<u8> {
        let pk = Ed25519KeyPair::generate_pkcs8(&self.rand).unwrap();
        pk.as_ref().to_vec()
    }

    pub fn sign(&self, keypair: &[u8], message: &[u8]) -> Vec<u8> {
        let kp = Ed25519KeyPair::from_pkcs8(keypair).unwrap();
        kp.sign(message).as_ref().to_vec()
    }

    pub fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        let pub_key = UnparsedPublicKey::new(&ED25519, public_key);
        pub_key.verify(message, signature).is_ok()
    }

    pub fn get_public_key(&self, keypair: &[u8]) -> Vec<u8> {
        let kp = Ed25519KeyPair::from_pkcs8(keypair).unwrap();
        kp.public_key().as_ref().to_vec()
    }
}
