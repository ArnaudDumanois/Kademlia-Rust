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

    pub fn add_node(&mut self, node_id: NodeId) {
        // Calculer la distance avec le nœud local
        let distance = xor_distance(node_id, self.local_node_id);
        // Déterminer l'index du K-bucket approprié en fonction de la distance
        let bucket_index = self.get_bucket_index(distance);
        println!("Adding node {:?} to bucket {}", node_id, bucket_index);
        // Ajouter le nœud au K-bucket déterminé
        self.buckets[bucket_index].add_node(node_id);
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        let bucket_index = self.get_bucket_index(xor_distance(node_id, self.local_node_id)); // Utiliser la distance pour déterminer le bucket
        self.buckets[bucket_index].remove_node(node_id);
    }

    pub fn get_bucket_index(&self, distance: NodeId) -> usize {
        // Trouver l'index du bucket en fonction de la distance
        for i in 0..self.buckets.len() {
            let min_distance = 1 << i; // 2^i
            let max_distance = 1 << (i + 1); // 2^(i+1)
            if distance >= min_distance && distance < max_distance {
                return i; // Retourne l'index du bucket correspondant
            }
        }
        // Si aucune condition n'est remplie, retourne le dernier bucket
        self.buckets.len() - 1
    }

    pub fn fill_bucket(&mut self, nodes: Vec<NodeId>) {
        // Remplir le K-bucket avec les nœuds fournis
        for node in nodes {
            println!("Adding node {:?} to routing table", node);
            self.add_node(node);
        }
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
