use crate::node::KademliaNode;
use crate::routing::bucket::KBucket;
use crate::utils::id::{NodeId, xor_distance};
use crate::tree::{TreeNode, insert_node};


pub struct RoutingTable {
    pub buckets: Vec<KBucket>,
    pub id_tree: Option<Box<TreeNode>>
}

impl RoutingTable {
    pub fn new() -> Self {
        RoutingTable {
            buckets: (0..16).map(|_| KBucket::new()).collect(),
            id_tree: None,
        }
    }

    pub fn add_node(&mut self, node: KademliaNode) {
        let bucket_index = self.get_bucket_index(node.id);
        self.buckets[bucket_index].add_node(node);
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        let bucket_index = self.get_bucket_index(node_id);
        self.buckets[bucket_index].remove_node(node_id);
    }

    pub fn get_bucket_index(&self, node_id: NodeId) -> usize {
        let distance = xor_distance(node_id, self.id_tree.as_ref().unwrap().id);
        let index = distance % 16;
        index as usize
    }

    pub fn find_closest_nodes(&self, target_id: &NodeId, count: usize) -> Vec<&KademliaNode> {
        let mut nodes: Vec<&KademliaNode> = self.buckets
            .iter()
            .flat_map(|bucket| bucket.get_nodes())
            .collect();

        nodes.sort_by_key(|node| xor_distance(node.id, *target_id));
        nodes.into_iter().take(count).collect()
    }
}
