use std::{env, process::exit};
use tree::Tree;
use node::Node;
pub mod node;
pub mod nodes;
pub mod tree;
pub mod traverse;

fn main() {
    // 1-GetPathAsString::begin
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide directory so start traversal.");
        std::process::exit(1);
    }
    let path: String = args[1].parse().expect("Please provide a valid String.");

    // 3-BuildTree::begin
    let vec: Option<Vec<Node>> = Tree::build_tree(&path);
    let tree: Option<Tree> = if let Some(vec) = vec {
        Some(Tree::new(&path, &vec))
    } else {
        None
    };

    if tree.is_none() {
        eprintln!("Error building tree");
        exit(1);
    }
    else {
        tree.unwrap().print()
    }
}