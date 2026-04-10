//! 内存池清理 - 过期交易删除、资源优化
use super::tx_pool::TxPool;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MempoolCleaner {
    max_age: u64,
    max_size: usize,
}

impl MempoolCleaner {
    pub fn new(max_age_minutes: u64, max_size: usize) -> Self {
        Self {
            max_age: max_age_minutes * 60 * 1000,
            max_size,
        }
    }

    fn current_timestamp() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
    }

    pub fn clean_expired(&self, pool: &mut TxPool) -> usize {
        let now = Self::current_timestamp();
        let expired: Vec<String> = pool.txs.iter()
            .filter(|(_, tx)| now - tx.timestamp > self.max_age)
            .map(|(h, _)| h.clone())
            .collect();
        
        let count = expired.len();
        for hash in expired {
            pool.remove_tx(&hash);
        }
        count
    }

    pub fn enforce_size_limit(&self, pool: &mut TxPool) -> usize {
        if pool.txs.len() <= self.max_size {
            return 0;
        }
        let overflow = pool.txs.len() - self.max_size;
        let oldest: Vec<String> = pool.txs.iter()
            .take(overflow)
            .map(|(h, _)| h.clone())
            .collect();
        
        for hash in oldest {
            pool.remove_tx(&hash);
        }
        overflow
    }

    pub fn full_clean(&self, pool: &mut TxPool) -> usize {
        let expired = self.clean_expired(pool);
        let overflow = self.enforce_size_limit(pool);
        expired + overflow
    }
}
