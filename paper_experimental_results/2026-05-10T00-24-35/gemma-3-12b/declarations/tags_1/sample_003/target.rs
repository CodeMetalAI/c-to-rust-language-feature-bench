#[derive(Debug)]
struct TNode {
    count: i32,
    left: Option<Box<TNode>>,
    right: Option<Box<TNode>>,
}

fn main() {
    let mut s = TNode {
        count: 1,
        left: None,
        right: None,
    };

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

    s.left = Some(Box::new(left_child));
    s.right = Some(Box::new(right_child));

    let sp = &s;

    if let Some(ref left) = sp.left {
        if left.count != 2 {
            return 1;
        }
    } else {
        return 1;
    }

    if let Some(ref right) = sp.right {
        if right.count != 3 {
            return 2;
        }
    } else {
        return 2;
    }

    if sp.left.as_ref().map(|x| x.as_ref() as *const TNode) == sp.right.as_ref().map(|x| x.as_ref() as *const TNode) {
        return 3;
    }

    0
}