//! 交易内存池 - 待打包交易、手续费排序、去重
use std::collections::{HashMap, BTreeSet};
use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Transaction {
    pub hash: String,
    pub fee: u64,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

pub struct TxPool {
    txs: HashMap<String, Transaction>,
    sorted_txs: BTreeSet<Reverse<(u64, String)>>,
    max_size: usize,
}

impl TxPool {
    pub fn new(max_size: usize) -> Self {
        Self {
            txs: HashMap::new(),
            sorted_txs: BTreeSet::new(),
            max_size,
        }
    }

    pub fn add_tx(&mut self, tx: Transaction) -> bool {
        if self.txs.contains_key(&tx.hash) || self.txs.len() >= self.max_size {
            return false;
        }
        self.sorted_txs.insert(Reverse((tx.fee, tx.hash.clone())));
        self.txs.insert(tx.hash.clone(), tx);
        true
    }

    pub fn remove_tx(&mut self, tx_hash: &str) -> Option<Transaction> {
        let tx = self.txs.remove(tx_hash)?;
        self.sorted_txs.remove(&Reverse((tx.fee, tx.hash.clone())));
        Some(tx)
    }

    pub fn get_top_txs(&self, count: usize) -> Vec<Transaction> {
        self.sorted_txs.iter()
            .take(count)
            .filter_map(|r| self.txs.get(&r.1))
            .cloned()
            .collect()
    }

    pub fn clear(&mut self) {
        self.txs.clear();
        self.sorted_txs.clear();
    }
}
