use std::collections::HashSet;

pub type NodeId = u8; // Maintenant, NodeId est un u8

pub fn generate_random_id(existing_ids: &mut HashSet<NodeId>) -> NodeId {
    loop {
        let id = rand::random::<u8>() % 64; // Génère un ID aléatoire entre 0 et 63
        if !existing_ids.contains(&id) {
            existing_ids.insert(id);
            return id; // Retourne l'ID unique
        }
    }
}

pub fn xor_distance(id1: NodeId, id2: NodeId) -> NodeId {
    id1 ^ id2 // Opération XOR entre deux identifiants de 6 bits
}
