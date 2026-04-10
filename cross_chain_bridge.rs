//! 跨链桥 - 异构链资产转移、消息验证
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CrossChainTx {
    pub source_chain: String,
    pub target_chain: String,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub nonce: u64,
    pub signature: Vec<u8>,
}

pub struct CrossChainBridge {
    supported_chains: Vec<String>,
    pending_txs: VecDeque<CrossChainTx>,
    relayers: Vec<String>,
    chain_id: u32,
}

impl CrossChainBridge {
    pub fn new(chain_id: u32) -> Self {
        Self {
            supported_chains: vec!["ETH".to_string(), "BSC".to_string(), "SOL".to_string(), "TRON".to_string()],
            pending_txs: VecDeque::new(),
            relayers: Vec::new(),
            chain_id,
        }
    }

    pub fn add_relayer(&mut self, relayer: String) {
        self.relayers.push(relayer);
    }

    pub fn lock_asset(&mut self, tx: CrossChainTx) -> bool {
        if !self.supported_chains.contains(&tx.target_chain) || tx.amount == 0 {
            return false;
        }
        self.pending_txs.push_back(tx);
        true
    }

    pub fn unlock_asset(&mut self, tx: &CrossChainTx) -> bool {
        self.pending_txs.iter().any(|t| t.nonce == tx.nonce)
    }

    pub fn process_pending(&mut self) -> Vec<CrossChainTx> {
        let mut processed = Vec::new();
        while let Some(tx) = self.pending_txs.pop_front() {
            processed.push(tx);
        }
        processed
    }
}
