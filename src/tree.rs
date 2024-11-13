use walkdir::WalkDir;
use crate::node::Node;
use crate::nodes::Nodes;

#[derive(Copy, Clone)]
pub enum MoveSelection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Tree {
    root: String,
    nodes: Nodes,
    pub selection: Option<usize>,
}

impl Tree {
    // Fn build_tree generates a vector of Nodes.
    // This output is used as an input to constructor `new`.
    pub fn build_tree(root: &String) -> Option<Vec<Node>> {
        let mut tree_vector: Vec<Node> = Vec::new();
        for entry in WalkDir::new(root) {
            match entry {
                Ok(e) => {
                    let e_path = e.path().to_str().unwrap();
                    let e_path = String::from(e_path);
                    let e_node = Node::new(e_path.clone());
                    tree_vector.push(e_node);

                    // If this node has a parent, then add the parent node as a parent to this node::begin.
                    if e.path().parent().is_some() {
                        // Get the parent ID.
                        let parent = e.path().parent().unwrap();
                        let parent = parent.to_str().unwrap();
                        let parent = String::from(parent);

                        // Find child's index into vector.
                        let child_idx =
                            tree_vector
                                .iter()
                                .position(|x| x.path() == e_path);

                        if child_idx.is_some() {
                            tree_vector[child_idx.unwrap()].add_parent(parent);
                        }
                    }
                    //::end

                    // If this node has a parent, then add this node as a child to the parent::begin.
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
                    //::end
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return None;
                }
            }
        }
        return Some(tree_vector);
    }

    // Constructor.
    pub fn new(root: &String, vec: &Vec<Node>) -> Self {
        Self {
            root: root.clone(),
            nodes: Nodes::new(vec),
            selection: if vec.is_empty() {
                None
            }
            else {
                Some(0)
            },
        }
    }

    pub fn move_selection(&mut self, dir: MoveSelection) -> bool {
        false
    }

    pub fn print(&self) {
        self.nodes.print();
    }
}