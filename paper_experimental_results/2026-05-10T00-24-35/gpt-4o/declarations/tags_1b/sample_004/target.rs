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

    let mut s = TNode {
        count: 1,
        left: Some(Box::new(left_child)),
        right: Some(Box::new(right_child)),
    };

    let sp = &mut s;

    if let Some(left) = &sp.left {
        if left.count != 2 {
            return 1;
        }
    }

    if let Some(right) = &sp.right {
        if right.count != 3 {
            return 2;
        }
    }

    if sp.left.is_some() && sp.right.is_some() && sp.left.as_ref().unwrap() as *const _ == sp.right.as_ref().unwrap() as *const _ {
        return 3;
    }

    0
}