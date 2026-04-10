//! 区块序列化/反序列化 - 二进制存储、网络传输
use bincode::{serialize, deserialize};
use super::blockchain_core::Block;

pub struct BlockSerializer;

impl BlockSerializer {
    pub fn to_bytes(block: &Block) -> Vec<u8> {
        serialize(block).unwrap_or_default()
    }

    pub fn from_bytes(data: &[u8]) -> Option<Block> {
        deserialize(data).ok()
    }

    pub fn to_hex(block: &Block) -> String {
        let bytes = Self::to_bytes(block);
        hex::encode(bytes)
    }

    pub fn from_hex(hex_str: &str) -> Option<Block> {
        let bytes = hex::decode(hex_str).ok()?;
        Self::from_bytes(&bytes)
    }

    pub fn validate_bytes(data: &[u8]) -> bool {
        deserialize::<Block>(data).is_ok()
    }
}
