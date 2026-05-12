use std::process::exit;

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

    let mut s = TNode {
        count: 1,
        left: None,
        right: None,
    };

    s.left = Some(&left_child);
    s.right = Some(&right_child);

    let sp = &s;

    if sp.left.unwrap().count != 2 {
        exit(1);
    }

    if s.right.unwrap().count != 3 {
        exit(2);
    }

    if std::ptr::eq(sp.left.unwrap(), sp.right.unwrap()) {
        exit(3);
    }
}