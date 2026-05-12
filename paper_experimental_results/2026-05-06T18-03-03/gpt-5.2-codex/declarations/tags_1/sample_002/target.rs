use std::ptr;

struct TNode<'a> {
    count: i32,
    left: Option<&'a TNode<'a>>,
    right: Option<&'a TNode<'a>>,
}

fn main() {
    let left_child = TNode {
        count: 2,
        left: None,
        right: None,
    };
    let right_child = TNode {
        count: 3,
        left: None,
        right: None,
    };
    let s = TNode {
        count: 1,
        left: Some(&left_child),
        right: Some(&right_child),
    };

    let sp = &s;

    if sp.left.unwrap().count != 2 {
        std::process::exit(1);
    }

    if s.right.unwrap().count != 3 {
        std::process::exit(2);
    }

    if ptr::eq(sp.left.unwrap(), sp.right.unwrap()) {
        std::process::exit(3);
    }
}