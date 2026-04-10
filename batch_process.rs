//! 批量交易处理 - 高并发区块打包优化
use super::tx_pool::{TxPool, Transaction};
use super::merkle_tree::MerkleTree;
use std::collections::VecDeque;

pub struct BatchProcessor {
    batch_size: usize,
    max_gas: u64,
}

impl BatchProcessor {
    pub fn new(batch_size: usize, max_gas: u64) -> Self {
        Self {
            batch_size,
            max_gas,
        }
    }

    pub fn create_batch(&self, pool: &TxPool) -> Vec<Transaction> {
        let mut batch = pool.get_top_txs(self.batch_size);
        let mut total_gas = 0;
        batch.retain(|tx| {
            total_gas += tx.fee;
            total_gas <= self.max_gas
        });
        batch
    }

    pub fn batch_to_merkle_root(&self, batch: &[Transaction]) -> String {
        let tx_hashes: Vec<String> = batch.iter().map(|tx| tx.hash.clone()).collect();
        let tree = MerkleTree::new(&tx_hashes);
        tree.get_root().to_string()
    }

    pub fn validate_batch(&self, batch: &[Transaction]) -> bool {
        !batch.is_empty() && batch.len() <= self.batch_size
    }

    pub fn process_batch(&self, mut pool: TxPool) -> (Vec<Transaction>, TxPool) {
        let batch = self.create_batch(&pool);
        for tx in &batch {
            pool.remove_tx(&tx.hash);
        }
        (batch, pool)
    }
}
