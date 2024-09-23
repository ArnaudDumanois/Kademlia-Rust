mod node;
mod routing;
mod utils;
mod network;

use node::KademliaNode;

fn main() {
    let mut node1 = KademliaNode::new(vec![1, 2, 3, 4]);
    let node2 = KademliaNode::new(vec![5, 6, 7, 8]);

    // Ajouter le nœud 2 à la table de routage de node1
    node1.add_node(node2.clone());

    // Afficher les nœuds dans la table de routage
    for node in node1.routing_table.get_nodes() {
        println!("Node ID: {:?}", node.id);
    }
}
