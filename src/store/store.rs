use std::collections::HashMap;

pub struct Store {
    data: HashMap<String, String>, // Exemple pour les paires clé-valeur
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        self.data.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut store = Store::new();
        store.insert("key".to_string(), "value".to_string());
        assert_eq!(store.data.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_get() {
        let mut store = Store::new();
        store.insert("key".to_string(), "value".to_string());
        assert_eq!(store.get(&"key".to_string()), Some(&"value".to_string()));
    }
}
