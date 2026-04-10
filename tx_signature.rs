//! 交易签名标准化 - 多类型交易签名格式
use serde::{Serialize, Deserialize};
use super::crypto_ed25519::Ed25519Crypto;

#[derive(Serialize, Deserialize, Debug)]
pub enum TxType {
    Transfer,
    Stake,
    Unstake,
    ContractCall,
    Vote,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedTransaction {
    pub tx_type: TxType,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub nonce: u64,
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

pub struct TxSigner {
    crypto: Ed25519Crypto,
}

impl TxSigner {
    pub fn new() -> Self {
        Self {
            crypto: Ed25519Crypto::new(),
        }
    }

    pub fn sign_transaction(&self, keypair: &[u8], mut tx: SignedTransaction) -> SignedTransaction {
        let msg = self.get_sign_message(&tx);
        let sig = self.crypto.sign(keypair, &msg);
        let pub_key = self.crypto.get_public_key(keypair);
        tx.signature = sig;
        tx.public_key = pub_key;
        tx
    }

    pub fn verify_transaction(&self, tx: &SignedTransaction) -> bool {
        let msg = self.get_sign_message(tx);
        self.crypto.verify(&tx.public_key, &msg, &tx.signature)
    }

    fn get_sign_message(&self, tx: &SignedTransaction) -> Vec<u8> {
        serde_json::to_vec(&(&tx.tx_type, &tx.sender, &tx.receiver, &tx.amount, &tx.nonce, &tx.data)).unwrap()
    }
}
