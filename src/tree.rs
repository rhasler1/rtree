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
    nodes: Nodes,
    pub selection: Option<usize>,
}

impl Tree {
    pub fn new(vec: &Vec<Node>) -> Self {
        Self {
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