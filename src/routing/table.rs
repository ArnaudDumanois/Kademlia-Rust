use std::collections::HashSet;
use crate::node::KademliaNode;
use crate::routing::bucket::KBucket;
use crate::utils::id::{NodeId, xor_distance};


pub struct RoutingTable {
    pub local_node_id: NodeId,
    pub buckets: Vec<KBucket>,
}

impl RoutingTable {
    pub fn new(local_node_id: NodeId) -> Self {
        RoutingTable {
            local_node_id,
            buckets: (0..6).map(|_| KBucket::new()).collect(),
        }
    }



    pub fn remove_node(&mut self, node_id: NodeId) {
        let bucket_index = self.get_bucket_index(xor_distance(node_id, self.local_node_id)); // Utiliser la distance pour déterminer le bucket
        self.buckets[bucket_index].remove_node(node_id);
    }

    pub fn get_bucket_index(&self, distance: NodeId) -> usize {
        for i in 0..self.buckets.len() {
            let min_distance = 1 << i; // 2^i
            let max_distance = 1 << (i + 1); // 2^(i+1)
            if distance >= min_distance && distance < max_distance {
                return i;
            }
        }
        self.buckets.len() - 1
    }

    pub fn add_node(&mut self, node_id: NodeId) {
        let distance = xor_distance(node_id, self.local_node_id);
        let bucket_index = self.get_bucket_index(distance);
        self.buckets[bucket_index].add_node(node_id);
    }

    pub fn fill_bucket(&mut self, nodes: Vec<NodeId>) {
        for node in nodes {
            self.add_node(node);
        }
    }

    pub fn find_closest_nodes(&self, target_id: &NodeId, k: usize) -> Vec<NodeId> {
        let mut nodes_with_distance: Vec<(NodeId, NodeId)> = Vec::new();
        let mut seen_nodes: HashSet<NodeId> = HashSet::new(); // Utiliser un HashSet pour éviter les doublons

        for bucket in &self.buckets {
            for node_id in &bucket.nodes { // Itérer sur les nœuds sans les cloner
                // Si le nœud a déjà été vu, on passe à l'itération suivante
                if seen_nodes.contains(node_id) {
                    continue;
                }
                let distance = xor_distance(*node_id, *target_id);
                nodes_with_distance.push((*node_id, distance));
                seen_nodes.insert(*node_id); // Marquer ce nœud comme vu
            }
        }

        // Trier par distance
        nodes_with_distance.sort_by_key(|(_, distance)| *distance);

        // Prendre les k plus proches nœuds
        print!("Closest nodes: ");
        nodes_with_distance.iter().take(k).for_each(|(id, distance)| {
            print!("{} ", id);
        });
        println!();
        nodes_with_distance.iter().take(k).map(|(id, _)| id.clone()).collect()
    }

    pub fn display(&self) {
        // Afficher le contenu de chaque bucket avec la plage de distances
        for (i, bucket) in self.buckets.iter().enumerate() {
            let min_distance = 1 << i;           // Distance minimale pour ce bucket
            let max_distance = 1 << (i + 1);     // Distance maximale pour ce bucket
            println!(
                "Bucket {} (Distance: {} - {}): {:?}",
                i,
                min_distance,
                max_distance - 1,
                bucket.get_nodes()
            );
        }
    }
}
