//! 区块链数据裁剪 - 历史归档、空间优化
use super::blockchain_core::Block;
use std::collections::VecDeque;

pub struct ChainPruner {
    keep_blocks: u64,
    archive_path: String,
}

impl ChainPruner {
    pub fn new(keep_blocks: u64, archive_path: &str) -> Self {
        Self {
            keep_blocks,
            archive_path: archive_path.to_string(),
        }
    }

    pub fn should_prune(&self, chain: &VecDeque<Block>) -> bool {
        chain.len() as u64 > self.keep_blocks
    }

    pub fn prune_chain(&self, chain: &mut VecDeque<Block>) -> usize {
        if !self.should_prune(chain) {
            return 0;
        }
        let prune_count = chain.len() - self.keep_blocks as usize;
        let mut archived = Vec::with_capacity(prune_count);
        for _ in 0..prune_count {
            if let Some(block) = chain.pop_front() {
                archived.push(block);
            }
        }
        self.archive_blocks(&archived);
        prune_count
    }

    fn archive_blocks(&self, blocks: &[Block]) {
        let data = serde_json::to_vec(blocks).unwrap_or_default();
        let _ = std::fs::write(format!("{}/archive_{}.bin", self.archive_path, chrono::Utc::now().timestamp()), data);
    }

    pub fn get_keep_height(&self, current_height: u64) -> u64 {
        current_height.saturating_sub(self.keep_blocks)
    }
}
