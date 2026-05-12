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

    let s = TNode {
        count: 1,
        left: Some(&left_child),
        right: Some(&right_child),
    };

    let sp = &s;

    match sp.left {
        Some(l) if l.count == 2 => {}
        _ => exit(1),
    }

    match s.right {
        Some(r) if r.count == 3 => {}
        _ => exit(2),
    }

    if let (Some(l), Some(r)) = (sp.left, sp.right) {
        if std::ptr::eq(l, r) {
            exit(3);
        }
    }

    exit(0);
}