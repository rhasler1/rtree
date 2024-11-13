use walkdir::WalkDir;
use crate::node::Node;

pub struct Traverse {
    root: String,
}

impl Traverse {
    pub fn new(root: String) -> Self {
        Self {
            root,
        }
    }

    pub fn root_dir_id(&self) -> String {
        self.root.clone()
    }

    // Need to write a function to do a step traversal.
    // Inputs:
    // Left, Right to move selection between children.
    // Enter -> move focus node to selected child node.
    // Up -> move focus to parent Node.

    pub fn traverse_dir(&self) -> Option<Vec<Node>> {
        let mut tree_vector: Vec<Node> = Vec::new();
        for entry in WalkDir::new(&self.root) {
            match entry {
                Ok(e) => {
                    let e_path = e.path().to_str().unwrap();
                    let e_path = String::from(e_path);
                    let e_node = Node::new(e_path.clone());
                    tree_vector.push(e_node);

                    let parent_idx: Option<usize> = if let Some(parent_path) = e.path().parent() {
                        let parent_id = parent_path.to_str().unwrap().to_string();
                        let parent = tree_vector
                            .iter()
                            .enumerate()
                            .find(|(_, t)| t.path() == parent_id);
                        parent.map(|(idx, _)| idx) // Return the index if found, else None
                    } else {
                        None // No parent path
                    };
    
                    if parent_idx.is_some() {
                        // Add child id to parent node.
                        tree_vector[parent_idx.unwrap()].add_child(e_path.clone());
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return None;
                }
            }
        }
        return Some(tree_vector);
    }
}