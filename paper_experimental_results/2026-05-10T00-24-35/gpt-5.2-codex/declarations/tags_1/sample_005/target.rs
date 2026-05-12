use std::process;
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

    match sp.left {
        Some(l) => {
            if l.count != 2 {
                process::exit(1);
            }
        }
        None => process::exit(1),
    }

    match s.right {
        Some(r) => {
            if r.count != 3 {
                process::exit(2);
            }
        }
        None => process::exit(2),
    }

    match (sp.left, sp.right) {
        (Some(l), Some(r)) => {
            if ptr::eq(l, r) {
                process::exit(3);
            }
        }
        _ => {}
    }

    process::exit(0);
}