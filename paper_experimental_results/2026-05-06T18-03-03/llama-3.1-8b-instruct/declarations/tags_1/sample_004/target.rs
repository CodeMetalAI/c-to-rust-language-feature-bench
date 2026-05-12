// C11 6.7.2.3 — Tags (Example 1)

struct Tnode {
    count: i32,
    left: Option<Box<Tnode>>,
    right: Option<Box<Tnode>>,
}

fn main() {
    let mut s = Tnode {
        count: 1,
        left: None,
        right: None,
    };
    let mut sp = Some(Box::new(s));

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

    s.left = Some(Box::new(left_child));
    s.right = Some(Box::new(right_child));

    sp = Some(Box::new(s));

    if sp.as_ref().unwrap().left.as_ref().unwrap().count != 2 {
        return;
    }

    if s.right.as_ref().unwrap().count != 3 {
        return;
    }

    if sp.as_ref().unwrap().left.as_ref().unwrap() == sp.as_ref().unwrap().right.as_ref().unwrap() {
        return;
    }

    return Ok(());
}