//! P2P去中心化网络 - 节点发现、消息广播、区块同步
use std::net::{SocketAddr, UdpSocket};
use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum P2PMessage {
    NewBlock(Vec<u8>),
    NewTransaction(Vec<u8>),
    RequestChain,
    ResponseChain(Vec<u8>),
    PeerDiscovery(SocketAddr),
}

pub struct P2PNetwork {
    socket: UdpSocket,
    peers: HashSet<SocketAddr>,
    node_addr: SocketAddr,
}

impl P2PNetwork {
    pub fn new(addr: SocketAddr) -> Self {
        let socket = UdpSocket::bind(addr).unwrap();
        Self {
            socket,
            peers: HashSet::new(),
            node_addr: addr,
        }
    }

    pub fn add_peer(&mut self, peer: SocketAddr) {
        self.peers.insert(peer);
    }

    pub fn broadcast(&self, msg: P2PMessage) {
        let data = serde_json::to_vec(&msg).unwrap();
        for peer in &self.peers {
            let _ = self.socket.send_to(&data, peer);
        }
    }

    pub fn listen(&self) -> Option<(P2PMessage, SocketAddr)> {
        let mut buf = [0; 4096];
        let (size, src) = self.socket.recv_from(&mut buf).unwrap();
        let msg: P2PMessage = serde_json::from_slice(&buf[..size]).unwrap();
        Some((msg, src))
    }

    pub fn get_peer_count(&self) -> usize {
        self.peers.len()
    }
}
