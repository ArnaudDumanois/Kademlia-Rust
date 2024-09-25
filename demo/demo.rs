use kademlia_rust::KademliaNode;
use kademlia_rust::routing::RoutingTable;
fn create_node() -> KademliaNode {
    KademliaNode::new() // Utilise la méthode de création de nœud
}

fn test_ping() {
    let node1 = create_node();
    let node2 = create_node();

    // Simuler le ping entre deux nœuds
    let success = node1.ping(&node2);
    println!("Ping entre nœud {:?} et nœud {:?} : {}", node1.id, node2.id, success);
}

fn test_routing_table() {
    let mut routing_table = RoutingTable::new();
    let node1 = create_node();
    let node2 = create_node();


}

fn main() {
    // Exécute les tests de la table de routage et du ping
    println!("Démarrage des tests de Kademlia...");
    test_ping();
}
