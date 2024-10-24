use crate::node::Node;

// This structure contains a vector of type Node.
#[derive(Clone)]
pub struct Nodes {
    node_vec: Vec<Node>,
}

impl Nodes {
    pub fn new(vec: &Vec<Node>) -> Self {
        Self {
            node_vec: Self::create_vec(vec),
        }
    }

    fn create_vec(vec: &Vec<Node>) -> Vec<Node> {
        let mut items = Vec::new();
        for e in vec {
            let item = e.clone();
            items.push(item);
        }
        return items;
    }

    pub fn print(&self) {
        for node in &self.node_vec {
            node.print_children();
        }
    }
}