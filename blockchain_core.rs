//! 区块链主核心引擎 - 链初始化、区块管理、共识验证
use std::collections::VecDeque;
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub prev_hash: String,
    pub hash: String,
    pub data: String,
    pub nonce: u64,
}

pub struct BlockchainCore {
    pub chain: VecDeque<Block>,
    pub version: &'static str,
}

impl BlockchainCore {
    pub fn new() -> Self {
        let mut chain = VecDeque::new();
        let genesis = Self::create_genesis_block();
        chain.push_back(genesis);
        
        Self {
            chain,
            version: "rust-blockchain-v1.0.0",
        }
    }

    fn create_genesis_block() -> Block {
        let now = Utc::now().timestamp_millis() as u64;
        Block {
            index: 0,
            timestamp: now,
            prev_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            hash: "genesis-block-hash-rust-core".to_string(),
            data: "genesis-block-initialized".to_string(),
            nonce: 0,
        }
    }

    pub fn get_latest_block(&self) -> Option<&Block> {
        self.chain.back()
    }

    pub fn add_block(&mut self, mut block: Block) -> bool {
        let latest = self.get_latest_block().unwrap();
        if block.prev_hash != latest.hash || block.index != latest.index + 1 {
            return false;
        }
        self.chain.push_back(block);
        true
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let prev = &self.chain[i-1];
            if current.hash == prev.hash || current.prev_hash != prev.hash {
                return false;
            }
        }
        true
    }
}
