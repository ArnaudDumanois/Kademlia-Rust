// src/tree.rs

use crate::utils::id::NodeId;


pub struct TreeNode {
    pub id: NodeId,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(id: NodeId) -> Self {
        TreeNode {
            id,
            left: None,
            right: None,
        }
    }
}

// Fonction pour insérer un nœud dans l'arbre
pub fn insert_node(root: &mut Option<Box<TreeNode>>, new_node_id: NodeId) {
    match root {
        Some(ref mut node) => {
            if new_node_id < node.id {
                insert_node(&mut node.left, new_node_id);
            } else {
                insert_node(&mut node.right, new_node_id);
            }
        }
        None => {
            *root = Some(Box::new(TreeNode {
                id: new_node_id,
                left: None,
                right: None,
            }));
        }
    }
}
