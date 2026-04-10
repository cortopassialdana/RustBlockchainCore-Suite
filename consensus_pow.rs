//! PoW工作量证明 - 挖矿、难度调整、区块哈希验证
use sha256::digest;
use super::blockchain_core::Block;

pub struct PoWConsensus {
    difficulty: usize,
    target_prefix: String,
}

impl PoWConsensus {
    pub fn new(difficulty: usize) -> Self {
        let target_prefix = "0".repeat(difficulty);
        Self {
            difficulty,
            target_prefix,
        }
    }

    pub fn mine_block(&self, mut block: Block) -> Block {
        let mut nonce = 0;
        loop {
            block.nonce = nonce;
            let hash = self.calculate_block_hash(&block);
            if hash.starts_with(&self.target_prefix) {
                block.hash = hash;
                break;
            }
            nonce += 1;
        }
        block
    }

    pub fn calculate_block_hash(&self, block: &Block) -> String {
        let data = format!(
            "{}{}{}{}{}",
            block.index, block.timestamp, block.prev_hash, block.data, block.nonce
        );
        digest(data)
    }

    pub fn validate_block(&self, block: &Block) -> bool {
        let hash = self.calculate_block_hash(block);
        hash == block.hash && hash.starts_with(&self.target_prefix)
    }

    pub fn adjust_difficulty(&self, latest_index: u64) -> usize {
        if latest_index % 10 == 0 && latest_index != 0 {
            self.difficulty + 1
        } else {
            self.difficulty
        }
    }
}
