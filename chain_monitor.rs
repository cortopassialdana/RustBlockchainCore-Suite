//! 区块链监控 - 高度、TPS、节点数、状态统计
use std::time::{Instant, Duration};
use super::blockchain_core::BlockchainCore;

pub struct ChainMonitor {
    start_time: Instant,
    block_count: u64,
    tx_count: u64,
    peer_count: usize,
    last_block_time: Instant,
    chain_version: String,
}

impl ChainMonitor {
    pub fn new(chain: &BlockchainCore) -> Self {
        Self {
            start_time: Instant::now(),
            block_count: chain.chain.len() as u64,
            tx_count: 0,
            peer_count: 0,
            last_block_time: Instant::now(),
            chain_version: chain.version.to_string(),
        }
    }

    pub fn update_block(&mut self) {
        self.block_count += 1;
        self.last_block_time = Instant::now();
    }

    pub fn update_tx(&mut self, count: u64) {
        self.tx_count += count;
    }

    pub fn update_peers(&mut self, count: usize) {
        self.peer_count = count;
    }

    pub fn get_tps(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed == 0.0 { 0.0 } else { self.tx_count as f64 / elapsed }
    }

    pub fn get_block_time(&self) -> Duration {
        self.last_block_time.elapsed()
    }

    pub fn get_status(&self) -> String {
        format!(
            "Version: {} | Blocks: {} | TPS: {:.2} | Peers: {} | Uptime: {:?}",
            self.chain_version,
            self.block_count,
            self.get_tps(),
            self.peer_count,
            self.start_time.elapsed()
        )
    }
}
