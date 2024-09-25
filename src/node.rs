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
        let id = generate_random_id();
        KademliaNode {
            id,
            rpc_client: RpcClient,
            routing_table: RoutingTable::new(id),
            store: Store::new(),
        }
    }

    pub fn ping(&mut self, target_id: NodeId) -> bool {
        let response = self.rpc_client.ping(&self.id,&target_id);
        if response.success {
            self.add_node_to_routing_table(target_id);
        }
        response.success
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
}

