use crate::utils::id::NodeId;
use crate::node::KademliaNode;
pub struct RpcResponse {
    pub success: bool,
    pub message: String,
}

pub struct RpcClient;
impl RpcClient {
    pub fn ping(&self, node: &KademliaNode) -> RpcResponse {
        println!("Pinging node with ID: {:?}", node.id);
        // Simule le ping
        RpcResponse {
            success: true,
            message: format!("Node with ID {:?} responded", node.id),
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
}
