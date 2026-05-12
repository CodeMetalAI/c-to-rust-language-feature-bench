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

    s.count = 1;
    left_child.count = 2;
    right_child.count = 3;

    s.left = Some(Box::new(left_child));
    s.right = Some(Box::new(right_child));

    let sp = Some(Box::new(s));

    if sp.as_ref().unwrap().left.as_ref().unwrap().count != 2 {
        return;
    }

    if s.right.as_ref().unwrap().count != 3 {
        return;
    }

    if sp.as_ref().unwrap().left.as_ref().unwrap() == sp.as_ref().unwrap().right.as_ref().unwrap() {
        return;
    }
}