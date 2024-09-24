use crate::node::KademliaNode;
use crate::routing::bucket::KBucket;
use crate::utils::id::{NodeId, xor_distance};

pub struct RoutingTable {
    pub buckets: Vec<KBucket>,
}

impl RoutingTable {
    pub fn new() -> Self {
        RoutingTable {
            buckets: (0..16).map(|_| KBucket::new(8)).collect(), // Crée 16 buckets
        }
    }

    pub fn add_node(&mut self, node: KademliaNode) {
        // Logique pour ajouter un nœud dans le bon bucket
        let bucket_index = self.get_bucket_index(node.id); // Utilise l'ID u8
        self.buckets[bucket_index].add_node(node);
    }

    pub fn remove_node(&mut self, node_id: NodeId) { // Change le type pour NodeId
        // Logique pour retirer un nœud du bon bucket
        let bucket_index = self.get_bucket_index(node_id);
        self.buckets[bucket_index].remove_node(node_id);
    }

    pub fn get_bucket_index(&self, node_id: NodeId) -> usize { // Change le type pour NodeId
        // Utilise la distance XOR pour déterminer l'index du bucket
        let distance = xor_distance(&node_id, &self.get_closest_node_id());
        // Exemple simple : prend le premier bit de la distance pour l'index
        let index = distance % 16; // Utilise directement le modulo 16 sur la distance
        index as usize
    }

    pub fn get_closest_node_id(&self) -> NodeId {
        // Récupère l'ID du nœud le plus proche
        0 // Exemple avec un ID nul (u8)
    }
}
