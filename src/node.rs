use crate::routing::RoutingTable;
use crate::utils::id::{Node, NodeId};
use crate::network::rpc::RpcClient;


pub struct KademliaNode {
    pub id: NodeId,
    pub routing_table: RoutingTable,
    pub rpc_client: RpcClient,
}

impl KademliaNode {
    pub fn new(id: NodeId) -> Self {
        KademliaNode {
            id,
            routing_table: RoutingTable::new(),
            rpc_client: RpcClient,
        }
    }



    pub fn ping(&self, target: &Node) -> bool {
        let response = self.rpc_client.ping(target);
        println!("Ping response from node {:?}: {}", target.id, response.message);
        response.success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping() {

    }

    #[test]
    fn test_new() {
        let node_id = vec![0x01, 0x02, 0x03];
        let node = KademliaNode::new(node_id.clone());
        assert_eq!(node.id, node_id);
    }
}
