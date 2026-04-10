//! 世界状态根 - 账户状态哈希、链状态验证
use std::collections::HashMap;
use super::crypto_sha256::SHA256Hasher;

#[derive(Debug, Clone)]
pub struct AccountState {
    pub balance: u64,
    pub nonce: u64,
    pub code_hash: String,
    pub storage_root: String,
}

pub struct WorldState {
    accounts: HashMap<String, AccountState>,
    state_root: String,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            state_root: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        }
    }

    pub fn update_account(&mut self, address: String, state: AccountState) {
        self.accounts.insert(address, state);
        self.recalculate_root();
    }

    pub fn get_account(&self, address: &str) -> Option<&AccountState> {
        self.accounts.get(address)
    }

    fn recalculate_root(&mut self) {
        let mut data = Vec::new();
        for (addr, acc) in &self.accounts {
            data.push(format!("{}:{}:{}:{}", addr, acc.balance, acc.nonce, acc.storage_root));
        }
        let combined = data.join(",");
        self.state_root = hex::encode(SHA256Hasher::hash(combined.as_bytes()));
    }

    pub fn get_state_root(&self) -> &str {
        &self.state_root
    }

    pub fn verify_state(&self, root: &str) -> bool {
        self.state_root == root
    }
}
