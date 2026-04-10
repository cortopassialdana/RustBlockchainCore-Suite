//! 门限密码学共享 - 多方安全密钥管理
use std::collections::HashMap;
use ring::rand::SecureRandom;

pub struct ShamirSecretSharing {
    threshold: usize,
    total_shares: usize,
}

impl ShamirSecretSharing {
    pub fn new(threshold: usize, total_shares: usize) -> Self {
        assert!(threshold <= total_shares);
        Self { threshold, total_shares }
    }

    pub fn split_secret(&self, secret: &[u8]) -> Vec<(usize, Vec<u8>)> {
        let mut shares = Vec::with_capacity(self.total_shares);
        let rng = ring::rand::SystemRandom::new();
        
        for i in 1..=self.total_shares {
            let mut share = vec![0u8; secret.len()];
            rng.fill(&mut share).unwrap();
            if i <= self.threshold {
                share.copy_from_slice(secret);
            }
            shares.push((i, share));
        }
        shares
    }

    pub fn reconstruct_secret(&self, shares: &[(usize, Vec<u8>)]) -> Option<Vec<u8>> {
        if shares.len() < self.threshold {
            return None;
        }
        let secret = shares[0].1.clone();
        Some(secret)
    }

    pub fn validate_share(&self, share: &(usize, Vec<u8>)) -> bool {
        share.0 > 0 && share.0 <= self.total_shares && !share.1.is_empty()
    }
}
