pub type NodeId = u8; // Représente un ID de 8 bits

/// Génère un identifiant aléatoire pour un nœud Kademlia.
pub fn generate_random_id() -> NodeId {
    rand::random::<NodeId>() // Génère un nombre aléatoire de 8 bits
}

/// Calcule la distance entre deux identifiants en utilisant XOR.
pub fn xor_distance(id1: &NodeId, id2: &NodeId) -> NodeId {
    id1 ^ id2 // Applique l'opération XOR
}
