//! RPC接口服务 - 外部系统调用区块链功能
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct RpcRequest {
    pub method: String,
    pub params: HashMap<String, String>,
    pub id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct RpcResponse {
    pub result: Option<String>,
    pub error: Option<String>,
    pub id: u64,
}

pub struct RpcServer {
    port: u16,
    methods: HashMap<String, fn(HashMap<String, String>) -> Result<String, String>>,
}

impl RpcServer {
    pub fn new(port: u16) -> Self {
        let mut methods = HashMap::new();
        methods.insert("get_block".to_string(), |p| Ok(format!("block_{}", p.get("height").unwrap_or(&"0".to_string()))));
        methods.insert("get_balance".to_string(), |p| Ok(format!("balance_{}", p.get("address").unwrap_or(&"0x0".to_string()))));
        methods.insert("send_tx".to_string(), |_| Ok("tx_sent".to_string()));
        
        Self { port, methods }
    }

    pub fn handle_request(&self, req: RpcRequest) -> RpcResponse {
        match self.methods.get(&req.method) {
            Some(handler) => {
                match handler(req.params) {
                    Ok(res) => RpcResponse { result: Some(res), error: None, id: req.id },
                    Err(e) => RpcResponse { result: None, error: Some(e), id: req.id },
                }
            }
            None => RpcResponse { result: None, error: Some("method not found".to_string()), id: req.id },
        }
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn register_method(&mut self, method: String, handler: fn(HashMap<String, String>) -> Result<String, String>) {
        self.methods.insert(method, handler);
    }
}
