struct TNode {
    count: i32,
    left: Option<Box<TNode>>,
    right: Option<Box<TNode>>,
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
        left: Some(Box::new(left_child)),
        right: Some(Box::new(right_child)),
    };

    let sp = &s;

    if let Some(ref left) = sp.left {
        if left.count != 2 {
            std::process::exit(1);
        }
    }

    if let Some(ref right) = s.right {
        if right.count != 3 {
            std::process::exit(2);
        }
    }

    if sp.left.is_some() && sp.right.is_some() && sp.left.as_ref().unwrap() as *const _ == sp.right.as_ref().unwrap() as *const _ {
        std::process::exit(3);
    }

    std::process::exit(0);
}