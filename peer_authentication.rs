//! 节点身份认证 - 防恶意节点接入
use std::collections::HashSet;
use super::crypto_ed25519::Ed25519Crypto;

pub struct PeerAuth {
    trusted_pubkeys: HashSet<Vec<u8>>,
    crypto: Ed25519Crypto,
    require_auth: bool,
}

impl PeerAuth {
    pub fn new(require_auth: bool) -> Self {
        Self {
            trusted_pubkeys: HashSet::new(),
            crypto: Ed25519Crypto::new(),
            require_auth,
        }
    }

    pub fn add_trusted(&mut self, pubkey: Vec<u8>) {
        self.trusted_pubkeys.insert(pubkey);
    }

    pub fn authenticate(&self, pubkey: &[u8], challenge: &[u8], signature: &[u8]) -> bool {
        if !self.require_auth {
            return true;
        }
        if !self.trusted_pubkeys.contains(pubkey) {
            return false;
        }
        self.crypto.verify(pubkey, challenge, signature)
    }

    pub fn generate_challenge(&self) -> Vec<u8> {
        let mut buf = [0u8; 32];
        ring::rand::SystemRandom::new().fill(&mut buf).unwrap();
        buf.to_vec()
    }

    pub fn remove_trusted(&mut self, pubkey: &[u8]) {
        self.trusted_pubkeys.remove(pubkey);
    }

    pub fn trusted_count(&self) -> usize {
        self.trusted_pubkeys.len()
    }
}
