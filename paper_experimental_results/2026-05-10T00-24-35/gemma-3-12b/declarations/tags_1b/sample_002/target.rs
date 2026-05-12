#[derive(Debug)]
struct TNode {
    count: i32,
    left: Option<&'static TNode>,
    right: Option<&'static TNode>,
}

fn main() {
    let mut s = TNode {
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

    s.left = Some(&left_child);
    s.right = Some(&right_child);

    let sp = &s;

    if let Some(left) = sp.left {
        if left.count != 2 {
            return 1;
        }
    } else {
        return 1; // Should never happen, but Rust requires a match
    }

    if let Some(right) = sp.right {
        if right.count != 3 {
            return 2;
        }
    } else {
        return 2;
    }

    if sp.left == sp.right {
        return 3;
    }

    0
}