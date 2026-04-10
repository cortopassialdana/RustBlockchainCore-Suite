//! WASM智能合约虚拟机 - 沙箱执行、合约调用、资源限制
use wasmtime::*;
use std::collections::HashMap;

pub struct WasmVM {
    engine: Engine,
    store: Store<()>,
    module_cache: HashMap<String, Module>,
    gas_limit: u64,
}

impl WasmVM {
    pub fn new(gas_limit: u64) -> Self {
        let engine = Engine::default();
        let store = Store::new(&engine, ());
        Self {
            engine,
            store,
            module_cache: HashMap::new(),
            gas_limit,
        }
    }

    pub fn compile_contract(&mut self, contract_id: &str, wasm_bytes: &[u8]) -> Result<(), String> {
        let module = Module::new(&self.engine, wasm_bytes).map_err(|e| e.to_string())?;
        self.module_cache.insert(contract_id.to_string(), module);
        Ok(())
    }

    pub fn execute_contract(&mut self, contract_id: &str, method: &str, params: &[u8]) -> Result<Vec<u8>, String> {
        let module = self.module_cache.get(contract_id).ok_or("contract not found")?;
        let linker = Linker::new(&self.engine);
        let instance = linker.instantiate(&mut self.store, module).map_err(|e| e.to_string())?;
        
        let func = instance.get_typed_func::<(u64, u64), u64>(&mut self.store, method)
            .map_err(|e| e.to_string())?;
        
        let result = func.call(&mut self.store, (params.as_ptr() as u64, params.len() as u64))
            .map_err(|e| e.to_string())?;
        
        Ok(result.to_be_bytes().to_vec())
    }

    pub fn check_gas(&self, used_gas: u64) -> bool {
        used_gas <= self.gas_limit
    }
}
