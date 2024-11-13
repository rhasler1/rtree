#[derive(Clone)]
pub struct Node {
    path: String,                                   // Unique ID: Relative path to root. 
    children: Vec<String>,                          // Unique ID(s) of this Node's children.
    parent: Option<String>,                                 // Unique ID of parent node.
}

// TODO: figure out how to get parent information.

impl Node {
    // Constructor.
    pub fn new(path: String) -> Self {
        Self {
            path,
            children: Vec::new(),
            parent: None,
        }
    }

    // Add child to this Node; note: children are
    // referred to by ID, NOT through referencing.
    pub fn add_child(&mut self, child_id: String) {
        self.children.push(child_id)
    }

    pub fn add_parent(&mut self, parent_id: String) {
        self.parent = Some(parent_id);
    }

    // Returns a copy of path.
    pub fn path(&self) -> String {
        self.path.clone()
    }

    // Returns a copy of children.
    pub fn children(&self) -> Vec<String> {
        return self.children.clone()
    }

    // Print children of this Node.
    pub fn print_children(&self) {
        if !self.children().is_empty() {
            for child in self.children() {
                println!("{:?}", child)
            }
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.path() == other.path()
    }
}

impl Eq for Node {}