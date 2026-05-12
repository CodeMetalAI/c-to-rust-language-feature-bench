use std::process;
use std::ptr;

struct Tnode {
    count: i32,
    left: Option<&'static Tnode>,
    right: Option<&'static Tnode>,
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
        process::exit(1);
    }

    if s.right.unwrap().count != 3 {
        process::exit(2);
    }

    if ptr::eq(sp.left.unwrap(), sp.right.unwrap()) {
        process::exit(3);
    }

    process::exit(0);
}