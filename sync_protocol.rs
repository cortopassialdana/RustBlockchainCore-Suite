//! 区块同步协议 - 新节点快速同步全链数据
use std::collections::VecDeque;
use super::blockchain_core::Block;

pub enum SyncState {
    Idle,
    Requesting,
    Receiving,
    Verifying,
    Completed,
}

pub struct ChainSyncProtocol {
    state: SyncState,
    start_index: u64,
    end_index: u64,
    received_blocks: VecDeque<Block>,
    batch_size: u64,
}

impl ChainSyncProtocol {
    pub fn new(batch_size: u64) -> Self {
        Self {
            state: SyncState::Idle,
            start_index: 0,
            end_index: 0,
            received_blocks: VecDeque::new(),
            batch_size,
        }
    }

    pub fn start_sync(&mut self, local_height: u64, remote_height: u64) {
        self.state = SyncState::Requesting;
        self.start_index = local_height + 1;
        self.end_index = std::cmp::min(local_height + self.batch_size, remote_height);
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        if matches!(self.state, SyncState::Receiving) && block.index >= self.start_index && block.index <= self.end_index {
            self.received_blocks.push_back(block);
            return true;
        }
        false
    }

    pub fn next_batch(&mut self) -> Option<(u64, u64)> {
        if self.received_blocks.len() as u64 == self.batch_size || self.start_index >= self.end_index {
            self.start_index = self.end_index + 1;
            self.end_index = self.start_index + self.batch_size;
            return Some((self.start_index, self.end_index));
        }
        None
    }

    pub fn complete_sync(&mut self) -> Vec<Block> {
        self.state = SyncState::Completed;
        self.received_blocks.drain(..).collect()
    }
}
