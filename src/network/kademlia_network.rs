use std::collections::HashMap;
use crate::node::KademliaNode;
use crate::utils::id::NodeId;

pub struct KademliaNetwork {
    pub nodes: HashMap<NodeId, KademliaNode>,  // Map NodeId to KademliaNode
}

impl KademliaNetwork {
    pub fn new() -> Self {
        KademliaNetwork {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: KademliaNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn add_nodes(&mut self, nodes: Vec<KademliaNode>) {
        for node in nodes {
            self.add_node(node);
        }
    }

    pub fn get_node(&self, node_id: &NodeId) -> Option<&KademliaNode> {
        self.nodes.get(node_id)
    }
}
