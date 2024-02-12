use std::io;

// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Function to calculate the maximum depth of a binary tree
fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

// Function to build a binary tree from user input
fn build_tree() -> Option<Box<TreeNode>> {
    println!("Enter the value of the root node:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let root_val: i32 = input.trim().parse().expect("Invalid number");

    let mut root = Some(Box::new(TreeNode::new(root_val)));

    let mut queue = vec![root.as_mut()];
    while let Some(Some(node)) = queue.pop() {
        println!("Enter the left child value of {}: (or -1 if none)", node.val);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let left_val: i32 = input.trim().parse().expect("Invalid number");

        if left_val != -1 {
            node.left = Some(Box::new(TreeNode::new(left_val)));
            queue.push(node.left.as_mut());
        }

        println!("Enter the right child value of {}: (or -1 if none)", node.val);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let right_val: i32 = input.trim().parse().expect("Invalid number");

        if right_val != -1 {
            node.right = Some(Box::new(TreeNode::new(right_val)));
            queue.push(node.right.as_mut());
        }
    }

    root
}

fn main() {
    let root = build_tree();

    let depth = max_depth(&root);
    println!("The maximum depth of the binary tree is: {}", depth);
}
