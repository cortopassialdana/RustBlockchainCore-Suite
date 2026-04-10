//! 区块链全局错误处理 - 异常捕获、日志上报
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum ChainErrorType {
    BlockInvalid,
    ConsensusFailed,
    TxInvalid,
    SignatureFailed,
    NetworkError,
    StorageError,
    ContractError,
    SyncError,
}

impl fmt::Display for ChainErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChainErrorType::BlockInvalid => write!(f, "BlockInvalid"),
            ChainErrorType::ConsensusFailed => write!(f, "ConsensusFailed"),
            ChainErrorType::TxInvalid => write!(f, "TxInvalid"),
            ChainErrorType::SignatureFailed => write!(f, "SignatureFailed"),
            ChainErrorType::NetworkError => write!(f, "NetworkError"),
            ChainErrorType::StorageError => write!(f, "StorageError"),
            ChainErrorType::ContractError => write!(f, "ContractError"),
            ChainErrorType::SyncError => write!(f, "SyncError"),
        }
    }
}

#[derive(Debug)]
pub struct ChainError {
    pub error_type: ChainErrorType,
    pub message: String,
    pub module: &'static str,
    pub timestamp: u64,
}

impl fmt::Display for ChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] [{}] {}: {}", self.timestamp, self.module, self.error_type, self.message)
    }
}

impl Error for ChainError {}

pub struct ErrorHandler {
    error_log: Vec<ChainError>,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self {
            error_log: Vec::new(),
        }
    }

    pub fn throw(&mut self, error_type: ChainErrorType, message: &str, module: &'static str) -> ChainError {
        let timestamp = chrono::Utc::now().timestamp_millis() as u64;
        let err = ChainError {
            error_type,
            message: message.to_string(),
            module,
            timestamp,
        };
        self.error_log.push(err.clone());
        err
    }

    pub fn get_errors(&self) -> &[ChainError] {
        &self.error_log
    }

    pub fn clear(&mut self) {
        self.error_log.clear();
    }
}
