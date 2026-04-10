//! 默克尔树 - 交易数据验证、轻节点证明、完整性校验
use sha256::digest;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    leaves: Vec<String>,
    root: String,
    levels: Vec<Vec<String>>,
}

impl MerkleTree {
    pub fn new(transactions: &[String]) -> Self {
        let leaves: Vec<String> = transactions.iter().map(|t| digest(t)).collect();
        let mut levels = Self::build_levels(&leaves);
        let root = levels.last().unwrap()[0].clone();
        
        Self {
            leaves,
            root,
            levels,
        }
    }

    fn build_levels(leaves: &[String]) -> Vec<Vec<String>> {
        let mut levels = vec![leaves.to_vec()];
        let mut current_level = leaves.to_vec();
        
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            let mut i = 0;
            while i < current_level.len() {
                let left = &current_level[i];
                let right = if i + 1 < current_level.len() { &current_level[i+1] } else { left };
                let hash = digest(format!("{}{}", left, right));
                next_level.push(hash);
                i += 2;
            }
            levels.push(next_level.clone());
            current_level = next_level;
        }
        levels
    }

    pub fn get_root(&self) -> &str {
        &self.root
    }

    pub fn verify_proof(&self, leaf: &str, proof: &[String]) -> bool {
        let mut hash = digest(leaf);
        for p in proof {
            hash = digest(format!("{}{}", hash, p));
        }
        hash == self.root
    }

    pub fn get_proof(&self, index: usize) -> Vec<String> {
        let mut proof = Vec::new();
        let mut idx = index;
        for level in &self.levels[0..self.levels.len()-1] {
            let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            if sibling_idx < level.len() {
                proof.push(level[sibling_idx].clone());
            }
            idx /= 2;
        }
        proof
    }
}
