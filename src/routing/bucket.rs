use crate::utils::id::NodeId;
use crate::node::KademliaNode;

pub struct KBucket {
    pub nodes: Vec<KademliaNode>,
}

impl KBucket {
    pub fn new() -> Self {
        KBucket {
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: KademliaNode) {
        if self.nodes.len() < 4 {
            self.nodes.push(node);
        } else {
            // bucket is full
            // replace the oldest node with the new node, a logic that can be improved
            self.nodes.sort_by(|a, b| a.id.cmp(&b.id));
        }
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        self.nodes.retain(|n| n.id != node_id);
    }

    pub fn get_nodes(&self) -> &Vec<KademliaNode> {
        &self.nodes
    }
}

