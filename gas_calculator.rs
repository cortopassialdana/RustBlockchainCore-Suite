//! 燃料费计算 - 智能合约执行消耗统计
use std::collections::HashMap;
use super::contract_compiler::Instruction;

pub struct GasCalculator {
    base_gas: u64,
    instruction_cost: HashMap<Instruction, u64>,
    storage_cost: u64,
}

impl GasCalculator {
    pub fn new() -> Self {
        let mut instruction_cost = HashMap::new();
        instruction_cost.insert(Instruction::Load(0), 10);
        instruction_cost.insert(Instruction::Store(0), 20);
        instruction_cost.insert(Instruction::Add, 5);
        instruction_cost.insert(Instruction::Sub, 5);
        instruction_cost.insert(Instruction::Mul, 8);
        instruction_cost.insert(Instruction::Div, 10);
        instruction_cost.insert(Instruction::Call(String::new()), 100);
        instruction_cost.insert(Instruction::Return, 2);

        Self {
            base_gas: 21000,
            instruction_cost,
            storage_cost: 1000,
        }
    }

    pub fn calculate_tx_gas(&self, has_data: bool) -> u64 {
        if has_data { self.base_gas + 6800 } else { self.base_gas }
    }

    pub fn calculate_contract_gas(&self, instructions: &[Instruction], storage_write: bool) -> u64 {
        let mut gas = self.base_gas;
        for ins in instructions {
            gas += self.instruction_cost.get(ins).unwrap_or(&50);
        }
        if storage_write {
            gas += self.storage_cost;
        }
        gas
    }

    pub fn check_gas_limit(&self, gas_used: u64, gas_limit: u64) -> bool {
        gas_used <= gas_limit
    }

    pub fn get_refund(&self, gas_used: u64, gas_limit: u64) -> u64 {
        if gas_used > gas_limit { 0 } else { gas_limit - gas_used }
    }
}
