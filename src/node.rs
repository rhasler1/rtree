#[derive(Clone)]
pub struct Node {
    path: String,
    children: Vec<String>,
}

impl Node {
    // Constructor.
    pub fn new(path: String) -> Self {
        Self {
            path,
            children: Vec::new(),
        }
    }

    // Add child to this Node; note: children are
    // referred to by ID, NOT through referencing.
    pub fn add_child(&mut self, child_id: String) {
        self.children.push(child_id)
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