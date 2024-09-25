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

pub fn insert_node(root: &mut Option<Box<TreeNode>>, id: &NodeId, bit_index: usize) {
    if let Some(ref mut node) = root {
        // Vérifie que bit_index ne dépasse pas la taille en bits de l'id
        if id < &node.id {
            insert_node(&mut node.left, id, bit_index + 1);
        } else {
            insert_node(&mut node.right, id, bit_index + 1);
        }
    } else {
        // Crée un nouveau nœud
        *root = Some(Box::new(TreeNode::new(id.clone())));
    }
}
