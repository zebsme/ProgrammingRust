use std::collections::HashMap;

enum RoughTime {}
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    // Attention why use Box here?
    Object(Box<HashMap<String, Json>>),
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

fn main() {
    use BinaryTree::*;
    let _jupyter_tree = NonEmpty(Box::new(TreeNode {
        element: "jupyer",
        left: Empty,
        right: Empty,
    }));
    println!("Hello, world!");
}
