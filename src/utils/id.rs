pub type NodeId = u8; // Maintenant, NodeId est un u8

pub fn generate_random_id() -> NodeId {
    rand::random::<u8>() % 64 // Génère un ID aléatoire entre 0 et 63
}

pub fn xor_distance(id1: NodeId, id2: NodeId) -> NodeId {
    id1 ^ id2 // Opération XOR entre deux identifiants de 6 bits
}
