use crate::utils::id::NodeId;
use crate::node::KademliaNode;
pub struct RpcResponse {
    pub success: bool,
    pub message: String,
}

pub struct RpcClient;
impl RpcClient {
    pub fn ping(&self, node_id: &NodeId, target_id: &NodeId) -> RpcResponse {
        println!("Node with ID {:?} is pinging the target {:?}", node_id, target_id);
        // Simule le ping
        RpcResponse {
            success: true,
            message: format!("Node with ID {:?} responded", node_id),
        }
    }

    pub fn find_node(&self, node_id: NodeId) -> RpcResponse {
        println!("Finding node with ID: {:?}", node_id);
        // Simule la recherche d'un nœud
        RpcResponse {
            success: true,
            message: format!("Node with ID {:?} found", node_id),
        }
    }

    pub fn store(&self, key: String, value: String) -> RpcResponse {
        println!("Storing key: {} with value: {}", key, value);
        // Simule le stockage
        RpcResponse {
            success: true,
            message: format!("Stored {} -> {}", key, value),
        }
    }

    pub fn find_value(&self, key: String) -> RpcResponse {
        println!("Finding value for key: {}", key);
        // Simule la recherche d'une valeur
        RpcResponse {
            success: true,
            message: format!("Value for key {} found", key),
        }
    }
}
