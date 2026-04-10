//! 区块链启动引导 - 创世区块、网络初始化
use super::blockchain_core::{BlockchainCore, Block};
use super::p2p_network::P2PNetwork;
use std::net::SocketAddr;

pub struct ChainBootstrap {
    chain_id: u32,
    genesis_data: String,
    boot_nodes: Vec<SocketAddr>,
}

impl ChainBootstrap {
    pub fn new(chain_id: u32) -> Self {
        Self {
            chain_id,
            genesis_data: format!("rust-blockchain-genesis-{}", chain_id),
            boot_nodes: vec![
                "127.0.0.1:8080".parse().unwrap(),
                "127.0.0.1:8081".parse().unwrap(),
            ],
        }
    }

    pub fn initialize_chain(&self) -> BlockchainCore {
        let mut core = BlockchainCore::new();
        let genesis = core.get_latest_block().cloned().unwrap();
        core.add_block(genesis);
        core
    }

    pub fn initialize_network(&self, local_addr: SocketAddr) -> P2PNetwork {
        let mut network = P2PNetwork::new(local_addr);
        for peer in &self.boot_nodes {
            if *peer != local_addr {
                network.add_peer(*peer);
            }
        }
        network
    }

    pub fn get_chain_id(&self) -> u32 {
        self.chain_id
    }

    pub fn add_boot_node(&mut self, addr: SocketAddr) {
        self.boot_nodes.push(addr);
    }
}
