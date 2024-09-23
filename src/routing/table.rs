use crate::node::KademliaNode;
use crate::routing::bucket::KBucket;
use crate::utils::id::{Node, NodeId, xor_distance};

pub struct RoutingTable {
    pub buckets: Vec<KBucket>,
}

impl RoutingTable {
    pub fn new() -> Self {
        RoutingTable {
            buckets: (0..16).map(|_| KBucket::new()).collect(),
        }
    }

    pub fn add_node(&mut self, node: KademliaNode) {
        // Logique pour ajouter un nœud dans le bon bucket
        let bucket_index = self.get_bucket_index(&node.id);
        self.buckets[bucket_index].add_node(node);
    }

    pub fn remove_node(&mut self, node_id: &Vec<u8>) {
        // Logique pour retirer un nœud du bon bucket
        self.buckets[0].remove_node(node_id); // Simplification pour l'exemple
    }

    pub fn get_bucket_index(&self, node_id: &NodeId) -> usize {
        // Utilise la distance XOR pour déterminer l'index du bucket
        let distance = xor_distance(node_id, &self.get_closest_node_id());
        // Exemple simple : prend le premier bit de la distance pour l'index
        let index = distance.iter().take(1).map(|byte| byte % 16).next().unwrap_or(0);
        index as usize
    }

    pub fn get_closest_node_id(&self) -> NodeId {
        // Récupère l'ID du nœud le plus proche
        vec![0; 32] // Exemple avec un ID nul
    }

    // Récupère tous les nœuds de la table de routage
    pub fn get_nodes(&self) -> Vec<KademliaNode> {
        self.buckets.iter().flat_map(|bucket| bucket.get_nodes().clone()).collect()
    }
}
