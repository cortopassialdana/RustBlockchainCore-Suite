//! 轻客户端 - 无需全链同步，快速验证交易
use super::merkle_tree::MerkleTree;
use super::crypto_sha256::SHA256Hasher;

#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub height: u64,
    pub prev_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
}

pub struct LightClient {
    headers: Vec<BlockHeader>,
    trusted_height: u64,
}

impl LightClient {
    pub fn new(trusted_genesis: BlockHeader) -> Self {
        Self {
            headers: vec![trusted_genesis],
            trusted_height: 0,
        }
    }

    pub fn add_header(&mut self, header: BlockHeader) -> bool {
        let last = self.headers.last().unwrap();
        if header.height != last.height + 1 || header.prev_hash != self.get_last_hash() {
            return false;
        }
        self.headers.push(header);
        self.trusted_height = self.headers.last().unwrap().height;
        true
    }

    pub fn get_last_hash(&self) -> String {
        let last = self.headers.last().unwrap();
        let data = format!("{}{}{}{}", last.height, last.prev_hash, last.merkle_root, last.timestamp);
        hex::encode(SHA256Hasher::hash(data.as_bytes()))
    }

    pub fn verify_tx(&self, tx_hash: &str, proof: &[String], height: u64) -> bool {
        let header = self.headers.iter().find(|h| h.height == height)?;
        let mut hash = tx_hash.to_string();
        for p in proof {
            hash = hex::encode(SHA256Hasher::hash(format!("{}{}", hash, p).as_bytes()));
        }
        hash == header.merkle_root
    }

    pub fn get_trusted_height(&self) -> u64 {
        self.trusted_height
    }
}
