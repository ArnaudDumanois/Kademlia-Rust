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
    pub fn new() -> Self {
        KademliaNode {
            id: generate_random_id(),
            routing_table: RoutingTable::new(),
            rpc_client: RpcClient,
            store: Store::new(),
        }
    }

    pub fn ping(&self, target: &KademliaNode) -> bool {
        let response = self.rpc_client.ping(&target.id);
        println!("Ping response from node {:?}: {}", target.id, response.message);
        response.success
    }

    pub fn distance_to(&self, target_id: NodeId) -> NodeId {
        utils::id::xor_distance(self.id, target_id)
    }

    pub fn store_value(&mut self, key: NodeId, value: String) {
        self.store.put(key, value);
    }

    pub fn retrieve_value(&self, key: &NodeId) -> Option<String> {
        self.store.get(key).cloned()
    }

}

