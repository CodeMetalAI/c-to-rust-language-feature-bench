use std::process;

struct Node<'a> {
    count: i32,
    left: Option<&'a Node<'a>>,
    right: Option<&'a Node<'a>>,
}

fn main() {
    let left_child = Node {
        count: 2,
        left: None,
        right: None,
    };
    let right_child = Node {
        count: 3,
        left: None,
        right: None,
    };

    let s = Node {
        count: 1,
        left: Some(&left_child),
        right: Some(&right_child),
    };

    let sp = &s;

    if sp.left.unwrap().count != 2 {
        process::exit(1);
    }

    if s.right.unwrap().count != 3 {
        process::exit(2);
    }

    if std::ptr::eq(sp.left.unwrap(), sp.right.unwrap()) {
        process::exit(3);
    }

    process::exit(0);
}