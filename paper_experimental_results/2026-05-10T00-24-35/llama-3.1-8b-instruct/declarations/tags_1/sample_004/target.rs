// C11 6.7.2.3 — Tags (Example 1)

struct TNode {
    count: i32,
    left: Option<Box<TNode>>,
    right: Option<Box<TNode>>,
}

fn main() {
    let s = TNode {
        count: 1,
        left: None,
        right: None,
    };
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

    s.left = Some(Box::new(left_child));
    s.right = Some(Box::new(right_child));

    let sp = &s;

    if sp.left.as_ref().unwrap().count != 2 {
        return;
    }

    if s.right.as_ref().unwrap().count != 3 {
        return;
    }

    if sp.left.as_ref().unwrap() == sp.right.as_ref().unwrap() {
        return;
    }

    return;
}