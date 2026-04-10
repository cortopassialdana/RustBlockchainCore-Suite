//! PoS权益证明 - 质押、节点选举、出块权重
use std::collections::HashMap;

pub struct PoSValidator {
    pub address: String,
    pub stake: u64,
    pub reputation: u32,
    pub is_active: bool,
}

pub struct PoSConsensus {
    validators: HashMap<String, PoSValidator>,
    min_stake: u64,
    epoch_blocks: u64,
}

impl PoSConsensus {
    pub fn new(min_stake: u64, epoch_blocks: u64) -> Self {
        Self {
            validators: HashMap::new(),
            min_stake,
            epoch_blocks,
        }
    }

    pub fn register_validator(&mut self, address: String, stake: u64) -> bool {
        if stake < self.min_stake || self.validators.contains_key(&address) {
            return false;
        }
        self.validators.insert(address.clone(), PoSValidator {
            address,
            stake,
            reputation: 100,
            is_active: true,
        });
        true
    }

    pub fn add_stake(&mut self, address: &str, amount: u64) -> bool {
        if let Some(v) = self.validators.get_mut(address) {
            v.stake += amount;
            return true;
        }
        false
    }

    pub fn elect_proposer(&self) -> Option<&PoSValidator> {
        let active: Vec<&PoSValidator> = self.validators.values().filter(|v| v.is_active).collect();
        if active.is_empty() {
            return None;
        }
        let total_stake: u64 = active.iter().map(|v| v.stake).sum();
        let mut rand = (total_stake / 2) as usize;
        for v in active {
            rand = rand.saturating_sub(v.stake as usize);
            if rand == 0 {
                return Some(v);
            }
        }
        active.first().copied()
    }

    pub fn slash_validator(&mut self, address: &str) {
        if let Some(v) = self.validators.get_mut(address) {
            v.stake = v.stake.saturating_div(2);
            v.reputation = v.reputation.saturating_sub(20);
            if v.reputation < 50 {
                v.is_active = false;
            }
        }
    }
}
