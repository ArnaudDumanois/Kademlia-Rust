use kademlia_rust::KademliaNode;
use rand::Rng;
fn create_node() -> KademliaNode {
    KademliaNode::new() // Utilise la méthode de création de nœud
}

fn test_ping() {
    let node1 = create_node();
    let node2 = create_node();

}

fn test_kbucket() {
    let mut nodes = Vec::new();
    for _ in 0..10 {
        nodes.push(KademliaNode::new());
    }

    println!("Nodes in network:");
    print!("Node IDs: ");
    for node in &nodes {
        print!("{} ", node.id);
    }
    println!();

    // Sélectionner un nœud aléatoire dans la liste des nœuds
    let random_index = rand::thread_rng().gen_range(0..nodes.len());

    // Créer une liste des autres nœuds à partir des parties gauche et droite **avant** de muter `left`
    let mut other_nodes: Vec<_> = nodes
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != random_index) // Exclure le nœud sélectionné
        .map(|(_, node)| node.id)
        .collect();

    // Séparer la liste en deux parties après la création de `other_nodes`
    let (left, right) = nodes.split_at_mut(random_index + 1);
    let random_node = &mut left[random_index]; // Emprunt mutable sur le nœud sélectionné

    println!("Other nodes: {:?}", other_nodes);

    // Remplir les buckets du nœud sélectionné avec les autres nœuds
    random_node.routing_table.fill_bucket(other_nodes);

    // Afficher l'état final des K-buckets
    println!("Final state of K-buckets (random node {}):", random_index);
    random_node.routing_table.display();
}




fn main() {
    // Exécute les tests de la table de routage et du ping
    println!("Démarrage des tests de Kademlia...");
    test_kbucket();
}
