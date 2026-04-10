//! 创世区块构建器 - 自定义链参数、初始分配
use super::blockchain_core::Block;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct GenesisConfig {
    pub chain_id: u32,
    pub initial_supply: u64,
    pub consensus: String,
    pub block_time: u64,
    pub genesis_address: String,
}

impl Default for GenesisConfig {
    fn default() -> Self {
        Self {
            chain_id: 1024,
            initial_supply: 1_000_000_000,
            consensus: "PoW+PoS".to_string(),
            block_time: 3000,
            genesis_address: "0x0000000000000000000000000000000000000000".to_string(),
        }
    }
}

pub struct GenesisBuilder {
    config: GenesisConfig,
}

impl GenesisBuilder {
    pub fn new(config: GenesisConfig) -> Self {
        Self { config }
    }

    pub fn build_genesis_block(&self) -> Block {
        let timestamp = Utc::now().timestamp_millis() as u64;
        let data = format!(
            "genesis:chain_id={}:supply={}:consensus={}:block_time={}",
            self.config.chain_id, self.config.initial_supply, self.config.consensus, self.config.block_time
        );
        
        Block {
            index: 0,
            timestamp,
            prev_hash: "0".repeat(64),
            hash: self.calculate_genesis_hash(&data),
            data,
            nonce: 0,
        }
    }

    fn calculate_genesis_hash(&self, data: &str) -> String {
        use sha256::digest;
        format!("genesis_{}", digest(data))
    }

    pub fn get_config(&self) -> &GenesisConfig {
        &self.config
    }

    pub fn export_config(&self) -> String {
        serde_json::to_string_pretty(&self.config).unwrap_or_default()
    }
}
