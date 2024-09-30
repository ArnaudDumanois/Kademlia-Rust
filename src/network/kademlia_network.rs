use std::collections::HashMap;
use rand::Rng;
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

    pub fn get_nodes(&self) -> Vec<&KademliaNode> {
        self.nodes.values().collect()
    }

    pub fn get_first_node(&self) -> Option<&KademliaNode> {
        self.nodes.values().next()
    }


    pub fn get_other_nodes(&self, node_id: &NodeId) -> Vec<&KademliaNode> {
        self.nodes.values().filter(|node| node.id != *node_id).collect()
    }

    pub fn get_random_node(&self) -> Option<&KademliaNode> {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.nodes.len());
        self.nodes.values().nth(index)
    }



    pub fn display(&self) {
        print!("Nodes in the network:");
        for (node_id, node) in &self.nodes {
            print!(" {:?},", node_id);
        }
        println!();
    }
}
