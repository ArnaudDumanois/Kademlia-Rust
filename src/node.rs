use crate::network::kademlia_network::KademliaNetwork;
use crate::routing::RoutingTable;
use crate::utils::id::{generate_random_id, NodeId};
use crate::utils;
use crate::network::rpc::RpcClient;
use crate::store::store::Store;

pub struct KademliaNode {
    pub id: NodeId,
    pub routing_table: RoutingTable,
    pub rpc_client: RpcClient,
    pub store: Store,
}

impl KademliaNode {
    pub fn new(id: NodeId) -> Self {
        let mut node = KademliaNode {
            id,
            rpc_client: RpcClient,
            routing_table: RoutingTable::new(id),
            store: Store::new(),
        };

        node
    }

    fn fill_initial_routing_table(&mut self) {

    }

    pub fn ping(&mut self, target_id: NodeId) -> bool {
        let response = self.rpc_client.ping(&self.id,&target_id);
        response.success
    }

    pub fn find_node(&mut self, target_id: NodeId, network: &KademliaNetwork) -> Option<NodeId> {
        // Étape 1 : Vérifier dans le K-Bucket local
        if self.routing_table.buckets.iter().any(|bucket| bucket.contains(&target_id)) {
            println!("Found in local K-bucket of node {:?}", self.id);
            return Some(target_id); // Si trouvé, retourne le node ID
        }

        // Étape 2 : Trouver les nœuds les plus proches
        let closest_nodes = self.routing_table.find_closest_nodes(&target_id, 4); // Par exemple, demander 3 nœuds les plus proches

        // Étape 3 : Interroger chaque nœud
        for node_id in closest_nodes {
            // Supposons que tu as une méthode dans RpcClient pour interroger un nœud
            let response = self.rpc_client.find_node(&node_id, &target_id, &network);
            if response.success {
                // Vérifie si le nœud cible est retourné
                if response.nodes.contains(&target_id) {
                    return Some(target_id); // Si trouvé, retourne le node ID
                }
            }
        }

        None // Si pas trouvé, retourne None
    }


    pub fn distance_to(&self, target_id: NodeId) -> NodeId {
        utils::id::xor_distance(self.id, target_id)
    }

    pub fn add_node_to_routing_table(&mut self, node_id: NodeId) {
        self.routing_table.add_node(node_id);
    }

    pub fn remove_node_from_routing_table(&mut self, node_id: NodeId) {
        self.routing_table.remove_node(node_id);
    }

    pub fn store_value(&mut self, key: NodeId, value: String) {
        self.store.put(key, value);
    }

    pub fn retrieve_value(&self, key: &NodeId) -> Option<String> {
        self.store.get(key).cloned()
    }

    pub fn fill_bucket(&mut self, nodes: Vec<NodeId>) {
        self.routing_table.fill_bucket(nodes);
    }
}

