struct TNode {
    count: i32,
    left: Option<Box<TNode>>,
    right: Option<Box<TNode>>,
}

fn main() {
    let mut left_child = TNode {
        count: 2,
        left: None,
        right: None,
    };

    let mut right_child = TNode {
        count: 3,
        left: None,
        right: None,
    };

    let mut s = TNode {
        count: 1,
        left: Some(Box::new(left_child)),
        right: Some(Box::new(right_child)),
    };

    let sp = &s;

    if sp.left.as_ref().unwrap().count != 2 {
        std::process::exit(1);
    }

    if sp.right.as_ref().unwrap().count != 3 {
        std::process::exit(2);
    }

    if std::ptr::eq(sp.left.as_ref().unwrap().as_ref(), sp.right.as_ref().unwrap().as_ref()) {
        std::process::exit(3);
    }

    std::process::exit(0);
}