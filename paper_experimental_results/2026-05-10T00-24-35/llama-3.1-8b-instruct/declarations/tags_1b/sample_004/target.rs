// This is a simple binary tree node implementation
struct TreeNode {
    count: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn main() {
    // Create the root node
    let mut s = TreeNode {
        count: 1,
        left: None,
        right: None,
    };

    // Create the left child node
    let left_child = TreeNode {
        count: 2,
        left: None,
        right: None,
    };

    // Create the right child node
    let right_child = TreeNode {
        count: 3,
        left: None,
        right: None,
    };

    // Assign the left and right children to the root node
    s.left = Some(Box::new(left_child));
    s.right = Some(Box::new(right_child));

    // Get a pointer to the root node
    let sp = &s;

    // Test the child count
    assert_eq!(sp.left.as_ref().unwrap().count, 2);

    // Test the sibling count
    assert_eq!(s.right.as_ref().unwrap().count, 3);

    // Test if the left child is the same as the right child
    assert_ne!(sp.left.as_ref().unwrap(), sp.right.as_ref().unwrap());

    // If all assertions pass, exit with 0
    std::process::exit(0);
}