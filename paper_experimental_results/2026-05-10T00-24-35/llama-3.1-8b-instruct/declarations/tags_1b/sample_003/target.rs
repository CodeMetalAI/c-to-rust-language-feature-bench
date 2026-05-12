// C11 6.7.2.3 — Tags (Example 1 Variant)

struct Node {
    count: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn main() {
    let s = Node {
        count: 1,
        left: Some(Box::new(Node {
            count: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(Node {
            count: 3,
            left: None,
            right: None,
        })),
    };

    let sp = &s;

    if sp.left.as_ref().unwrap().count != 2 {
        panic!("sp->left->count != 2");
    }

    if s.right.as_ref().unwrap().count != 3 {
        panic!("s.right->count != 3");
    }

    if sp.left.as_ref().unwrap() == sp.right.as_ref().unwrap() {
        panic!("sp->left == sp->right");
    }

    panic!("Program should not reach here");
}