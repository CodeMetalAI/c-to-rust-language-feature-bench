// C11 6.7.2.3 — Tags (Example 1)

struct TreeNode {
    count: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn main() {
    let mut s = TreeNode {
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

    s.left = Some(Box::new(left_child));
    s.right = Some(Box::new(right_child));

    let sp = &s;

    if sp.left.as_ref().unwrap().count != 2 {
        return;
    }

    if s.right.as_ref().unwrap().count != 3 {
        return;
    }

    if sp.left.as_ref().unwrap() == sp.right.as_ref().unwrap() {
        return;
    }

    panic!("Expected to return 3, but got");
}