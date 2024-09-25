mod node;
mod routing;
mod utils;
mod network;
mod store;

mod tree;

use node::KademliaNode;

fn main() {
    let node1 = KademliaNode::new();
    let node2 = KademliaNode::new();

    println!("ID de Node 1 : {:?}", node1.id);
    println!("ID de Node 2 : {:?}", node2.id);

    let distance = node1.distance_to(node2.id);
    println!("Distance entre Node 1 et Node 2 : {:?}", distance);
}

