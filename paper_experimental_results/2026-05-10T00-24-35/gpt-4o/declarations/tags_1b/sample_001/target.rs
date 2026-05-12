struct TNode {
    count: i32,
    left: Option<Box<TNode>>,
    right: Option<Box<TNode>>,
}

fn main() -> i32 {
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

    if sp.left.as_ref().unwrap().count != 2 {
        return 1;
    }

    if s.right.as_ref().unwrap().count != 3 {
        return 2;
    }

    if sp.left.as_ref().unwrap().count == sp.right.as_ref().unwrap().count {
        return 3;
    }

    0
}