//! UTXO账户模型 - 交易输入输出、双花防御、余额统计
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXO {
    pub tx_hash: String,
    pub index: u32,
    pub address: String,
    pub amount: u64,
    pub is_spent: bool,
}

pub struct UTXOManager {
    utxo_set: HashMap<String, UTXO>,
    address_utxos: HashMap<String, Vec<String>>,
}

impl UTXOManager {
    pub fn new() -> Self {
        Self {
            utxo_set: HashMap::new(),
            address_utxos: HashMap::new(),
        }
    }

    pub fn add_utxo(&mut self, utxo: UTXO) {
        let key = format!("{}:{}", utxo.tx_hash, utxo.index);
        self.address_utxos.entry(utxo.address.clone())
            .or_insert_with(Vec::new)
            .push(key.clone());
        self.utxo_set.insert(key, utxo);
    }

    pub fn spend_utxo(&mut self, tx_hash: &str, index: u32) -> bool {
        let key = format!("{}:{}", tx_hash, index);
        if let Some(utxo) = self.utxo_set.get_mut(&key) {
            if utxo.is_spent {
                return false;
            }
            utxo.is_spent = true;
            return true;
        }
        false
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.address_utxos.get(address)
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|k| self.utxo_set.get(k))
            .filter(|u| !u.is_spent)
            .map(|u| u.amount)
            .sum()
    }

    pub fn is_double_spend(&self, tx_hash: &str, index: u32) -> bool {
        let key = format!("{}:{}", tx_hash, index);
        self.utxo_set.get(&key).map_or(false, |u| u.is_spent)
    }
}
