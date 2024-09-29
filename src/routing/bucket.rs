use crate::utils::id::NodeId;
use crate::node::KademliaNode;

pub struct KBucket {
    pub nodes: Vec<NodeId>,
}

impl KBucket {
    pub fn new() -> Self {
        KBucket {
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: NodeId) {
        if self.nodes.len() < 4 {
            self.nodes.push(node);
        } else {
            self.nodes.sort();
            self.nodes.pop();
            self.nodes.push(node);
        }
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        self.nodes.retain(|&id| id != node_id);
    }

    pub fn get_nodes(&self) -> &Vec<NodeId> {
        &self.nodes
    }

    pub fn contains(&self, node_id: &NodeId) -> bool {
        self.nodes.contains(node_id)
    }
}

