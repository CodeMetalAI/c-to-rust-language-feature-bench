use std::process::exit;
use std::ptr;

#[derive(Copy, Clone)]
struct Tnode<'a> {
    count: i32,
    left: Option<&'a Tnode<'a>>,
    right: Option<&'a Tnode<'a>>,
}

fn main() {
    let left_child = Tnode {
        count: 2,
        left: None,
        right: None,
    };
    let right_child = Tnode {
        count: 3,
        left: None,
        right: None,
    };
    let s = Tnode {
        count: 1,
        left: Some(&left_child),
        right: Some(&right_child),
    };

    let sp = &s;

    if sp.left.unwrap().count != 2 {
        exit(1);
    }

    if s.right.unwrap().count != 3 {
        exit(2);
    }

    if ptr::eq(sp.left.unwrap(), sp.right.unwrap()) {
        exit(3);
    }
}