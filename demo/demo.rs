use kademlia_rust::KademliaNode;
use kademlia_rust::routing::RoutingTable;
fn create_node() -> KademliaNode {
    KademliaNode::new() // Utilise la méthode de création de nœud
}

fn test_ping() {
    let node1 = create_node();
    let node2 = create_node();

}

fn test() {
    let mut nodes = Vec::new();
    for _ in 0..10 {
        nodes.push(KademliaNode::new());
    }
    // Afficher l'état initial des K-buckets
    println!("Initial state of K-buckets:");
    for node in &nodes {
        println!("Node ID: {:?}", node.id);
        node.routing_table.display();
    }

    // Simuler l'ajout de nœuds dans les K-buckets
    for target_index in 0..nodes.len() {
        // Utiliser split_at_mut pour obtenir deux parties mutables
        let (left, right) = nodes.split_at_mut(target_index + 1);
        let target_node = &mut left[target_index]; // Emprunt mutable sur le nœud cible

        for i in 0..right.len() {
            let other_node = &right[i]; // Emprunt immuable sur le nœud autre
            if target_node.id != other_node.id {
                target_node.ping(other_node.id);
            }
        }
    }


    // Afficher l'état final des K-buckets
    println!("Final state of K-buckets:");
    for node in &nodes {
        println!("Node ID: {:?}", node.id);
        node.routing_table.display();
    }
}

fn main() {
    // Exécute les tests de la table de routage et du ping
    println!("Démarrage des tests de Kademlia...");
    test();
}
