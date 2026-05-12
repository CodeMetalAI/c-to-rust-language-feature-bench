/*
 * C11 6.7.2.3 — Tags (Example 1)
 */

struct TreeNode {
    count: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn main() {
    let mut root = TreeNode {
        count: 1,
        left: None,
        right: None,
    };

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

    root.left = Some(Box::new(left_child));
    root.right = Some(Box::new(right_child));

    let sp = &mut root;

    if sp.left.as_ref().unwrap().count != 2 {
        std::process::exit(1);
    }

    if root.right.as_ref().unwrap().count != 3 {
        std::process::exit(2);
    }

    if sp.left.as_ref().unwrap() == sp.right.as_ref().unwrap() {
        std::process::exit(3);
    }

    std::process::exit(0);
}