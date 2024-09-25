use std::collections::HashMap;
use crate::utils::id::NodeId;


pub struct Store {
    data: HashMap<NodeId, String>, // Exemple pour les paires clé-valeur
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: NodeId, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &NodeId) -> Option<&String> {
        self.data.get(key)
    }

    pub fn remove(&mut self, key: &NodeId) {
        self.data.remove(key);
    }

    pub fn contains_key(&self, key: &NodeId) -> bool {
        self.data.contains_key(key)
    }
}

