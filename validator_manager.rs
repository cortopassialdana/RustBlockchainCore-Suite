//! 验证节点管理 - 联盟链权限、出块调度
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ValidatorNode {
    pub address: String,
    pub power: u64,
    pub online: bool,
    pub missed_blocks: u32,
}

pub struct ValidatorManager {
    validators: HashMap<String, ValidatorNode>,
    max_missed: u32,
    total_power: u64,
}

impl ValidatorManager {
    pub fn new(max_missed: u32) -> Self {
        Self {
            validators: HashMap::new(),
            max_missed,
            total_power: 0,
        }
    }

    pub fn add_validator(&mut self, address: String, power: u64) -> bool {
        if self.validators.contains_key(&address) {
            return false;
        }
        self.total_power += power;
        self.validators.insert(address.clone(), ValidatorNode {
            address,
            power,
            online: true,
            missed_blocks: 0,
        });
        true
    }

    pub fn report_missed(&mut self, address: &str) {
        if let Some(v) = self.validators.get_mut(address) {
            v.missed_blocks += 1;
            if v.missed_blocks >= self.max_missed {
                v.online = false;
                self.total_power -= v.power;
            }
        }
    }

    pub fn elect_proposer(&self, height: u64) -> Option<&ValidatorNode> {
        let active: Vec<&ValidatorNode> = self.validators.values().filter(|v| v.online).collect();
        if active.is_empty() {
            return None;
        }
        let idx = (height % active.len() as u64) as usize;
        active.get(idx).copied()
    }

    pub fn get_active_count(&self) -> usize {
        self.validators.values().filter(|v| v.online).count()
    }
}
