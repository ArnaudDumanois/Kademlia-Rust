use std::collections::HashSet;
use kademlia_rust::KademliaNode;
use rand::Rng;
use kademlia_rust::utils::id::{generate_random_id, NodeId};
use kademlia_rust::network::kademlia_network::KademliaNetwork;

fn create_node(existing_ids: &mut HashSet<NodeId>) -> KademliaNode {
    let id = generate_random_id(existing_ids);
    KademliaNode::new(id) // Utilise la méthode de création de nœud
}

fn create_nodes(n: usize) -> Vec<KademliaNode> {
    let mut existing_ids = HashSet::new(); // Ensemble pour garder une trace des ID
    let mut nodes = (0..n).map(|_| create_node(&mut existing_ids)).collect::<Vec<KademliaNode>>();

    // Créer un vecteur des IDs des nœuds avant de remplir les K-Buckets
    let node_ids: Vec<NodeId> = nodes.iter().map(|node| node.id).collect();

    // Remplir les K-Buckets pour chaque nœud
    for node in &mut nodes {
        // Collecter les IDs des autres nœuds
        let other_nodes: Vec<NodeId> = node_ids.iter()
            .filter(|&&id| id != node.id) // Exclure le nœud lui-même
            .cloned() // Convertir de référence à valeur
            .collect();

        // Remplir le K-Bucket avec les autres nœuds
        node.fill_bucket(other_nodes);
    }

    nodes // Retourner la liste de nœuds
}



fn test_kbucket() {
    let mut nodes = Vec::new();
    for _ in 0..10 {
        nodes.push(KademliaNode::new(generate_random_id(&mut HashSet::new())));
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
    println!("Final state of K-buckets (random node {}):", random_node.id);
    random_node.routing_table.display();
}

fn test_find_node() {
    // Créer 15 nœuds
    let nodes = create_nodes(15);

    // Initialiser le réseau Kademlia et ajouter les nœuds
    let mut network = KademliaNetwork::new();

    // Ajouter chaque nœud au réseau
    network.add_nodes(nodes);

    network.display();

    // Sélectionner un nœud aléatoire pour la recherche
    let origin_node = network.get_first_node().unwrap();
    let target_node = network.get_random_node().unwrap();

    println!("Origin node: {:?}", origin_node.id);
    println!("Target node: {:?}", target_node.id);
    network.display();

    // Rechercher le nœud cible à partir du nœud d'origine
    let response = origin_node.find_node(target_node.id,&network);

}







fn main() {
    // Exécute les tests de la table de routage et du ping
    println!("Démarrage des tests de Kademlia...");
    test_find_node();
}
