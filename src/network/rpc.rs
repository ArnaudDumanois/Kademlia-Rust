use crate::utils::id::{Node, NodeId};
pub struct RpcResponse {
    pub success: bool,
    pub message: String,
}

pub struct RpcClient;
impl RpcClient {
    pub fn ping(&self, node: &Node) -> RpcResponse {
        println!("Pinging node: {:?}", node.id);
        // Simule une réponse de ping
        RpcResponse {
            success: true,
            message: format!("Node {} is reachable", node.id.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(",")),
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
