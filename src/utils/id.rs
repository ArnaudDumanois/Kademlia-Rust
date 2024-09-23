use std::net::SocketAddr;

pub type NodeId = Vec<u8>;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub addr: SocketAddr,
}

impl Node {
    pub fn new(id: NodeId, addr: SocketAddr) -> Self {
        Node { id, addr }
    }
}

pub fn xor_distance(id1: &NodeId, id2: &NodeId) -> NodeId {
    id1.iter()
        .zip(id2.iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect()
}
