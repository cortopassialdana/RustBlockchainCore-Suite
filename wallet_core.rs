//! 去中心化钱包 - 助记词、密钥、签名
use bip39::{Mnemonic, Language, Seed};
use ring::hmac;

pub struct BlockchainWallet {
    mnemonic: Mnemonic,
    seed: Seed,
    private_key: Vec<u8>,
    public_key: Vec<u8>,
    address: String,
}

impl BlockchainWallet {
    pub fn new() -> Self {
        let mnemonic = Mnemonic::generate_in(Language::English, 12).unwrap();
        let seed = Seed::new(&mnemonic, "");
        let private_key = Self::seed_to_private_key(&seed);
        let public_key = Self::private_to_public(&private_key);
        let address = Self::public_to_address(&public_key);
        
        Self {
            mnemonic,
            seed,
            private_key,
            public_key,
            address,
        }
    }

    pub fn from_mnemonic(phrase: &str) -> Self {
        let mnemonic = Mnemonic::parse_in(Language::English, phrase).unwrap();
        let seed = Seed::new(&mnemonic, "");
        let private_key = Self::seed_to_private_key(&seed);
        let public_key = Self::private_to_public(&private_key);
        let address = Self::public_to_address(&public_key);
        
        Self {
            mnemonic,
            seed,
            private_key,
            public_key,
            address,
        }
    }

    fn seed_to_private_key(seed: &Seed) -> Vec<u8> {
        let key = hmac::Key::new(hmac::HMAC_SHA512, seed.as_bytes());
        hmac::sign(&key, b"rust-blockchain-wallet").as_ref().to_vec()
    }

    fn private_to_public(private: &[u8]) -> Vec<u8> {
        use ring::signature::Ed25519KeyPair;
        let kp = Ed25519KeyPair::from_seed_unchecked(private).unwrap();
        kp.public_key().as_ref().to_vec()
    }

    fn public_to_address(public: &[u8]) -> String {
        let hash = ring::digest::digest(&ring::digest::SHA256, public);
        format!("0x{}", hex::encode(&hash.as_ref()[0..20]))
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_mnemonic_phrase(&self) -> String {
        self.mnemonic.to_string()
    }
}
