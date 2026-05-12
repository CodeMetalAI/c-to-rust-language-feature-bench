// Define a struct to represent a tree node
struct TreeNode {
    count: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn main() {
    // Create a new tree node
    let mut s = TreeNode {
        count: 1,
        left: None,
        right: None,
    };

    // Create child nodes
    let left_child = TreeNode {
        count: 2,
        left: None,
        right: None,
    };

    let right_child = TreeNode {
        count: 3,
        left: None,
        right: None,
    };

    // Set child nodes
    s.left = Some(Box::new(left_child));
    s.right = Some(Box::new(right_child));

    // Get a reference to the root node
    let sp = &s;

    // Check node counts
    if sp.left.as_ref().unwrap().count != 2 {
        println!("Error: left child count is incorrect");
        return;
    }

    if s.right.as_ref().unwrap().count != 3 {
        println!("Error: right child count is incorrect");
        return;
    }

    // Check if left and right child nodes are the same
    if sp.left.as_ref().unwrap() == sp.right.as_ref().unwrap() {
        println!("Error: left and right child nodes are the same");
        return;
    }

    println!("All checks passed");
}