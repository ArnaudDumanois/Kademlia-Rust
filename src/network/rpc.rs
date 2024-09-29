use crate::network::kademlia_network::KademliaNetwork;
use crate::utils::id::NodeId;
use crate::node::KademliaNode;
use crate::routing::RoutingTable;

pub struct RpcResponse {
    pub success: bool,
    pub message: String,
    pub nodes: Vec<NodeId>,
}

pub struct RpcClient;
impl RpcClient {
    pub fn ping(&self, node_id: &NodeId, target_id: &NodeId) -> RpcResponse {
        println!("Node with ID {:?} is pinging the target {:?}", node_id, target_id);
        // Simule le ping
        RpcResponse {
            success: true,
            message: format!("Node with ID {:?} responded", node_id),
            nodes: Vec::new(),
        }
    }

    pub fn find_node(&self, node_id: &NodeId, target_id: &NodeId, network: &KademliaNetwork) -> RpcResponse {
        println!("Finding node with ID: {:?} in the routing table of node {:?}", target_id, node_id);

        // Récupérer le KademliaNode à partir de son NodeId
        if let Some(kademlia_node) = network.get_node(node_id) {
            let closest_nodes = kademlia_node.routing_table.find_closest_nodes(target_id, 4);

            if closest_nodes.contains(target_id) {
                return RpcResponse {
                    success: true,
                    message: format!("Node {:?} found in the routing table of node {:?}", target_id, node_id),
                    nodes: closest_nodes,
                };
            }

            // Continuer la recherche dans les nœuds les plus proches
            for closest_node in closest_nodes {
                if let Some(next_kademlia_node) = network.get_node(&closest_node) {
                    let found_nodes = next_kademlia_node.routing_table.find_closest_nodes(target_id, 4);
                    if found_nodes.contains(target_id) {
                        return RpcResponse {
                            success: true,
                            message: format!("Node {:?} found in the routing table of node {:?}", target_id, closest_node),
                            nodes: found_nodes,
                        };
                    }
                }
            }
        }

        // Si aucune correspondance n'a été trouvée
        RpcResponse {
            success: false,
            message: format!("No nodes found for ID {:?}", target_id),
            nodes: Vec::new(),
        }
    }




    pub fn store(&self, key: String, value: String) -> RpcResponse {
        println!("Storing key: {} with value: {}", key, value);
        // Simule le stockage
        RpcResponse {
            success: true,
            message: format!("Stored {} -> {}", key, value),
            nodes: Vec::new(),
        }
    }

    pub fn find_value(&self, key: String) -> RpcResponse {
        println!("Finding value for key: {}", key);
        // Simule la recherche d'une valeur
        RpcResponse {
            success: true,
            message: format!("Value for key {} found", key),
            nodes: Vec::new(),
        }
    }
}
