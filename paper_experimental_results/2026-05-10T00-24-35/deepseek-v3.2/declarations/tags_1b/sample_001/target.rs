struct TNODE {
    count: i32,
    left: Option<Box<TNODE>>,
    right: Option<Box<TNODE>>,
}

fn main() {
    let left_child = TNODE {
        count: 2,
        left: None,
        right: None,
    };
    let right_child = TNODE {
        count: 3,
        left: None,
        right: None,
    };
    let mut s = TNODE {
        count: 1,
        left: Some(Box::new(left_child)),
        right: Some(Box::new(right_child)),
    };
    let sp = &s;

    if sp.left.as_ref().unwrap().count != 2 {
        return;
    }
    if s.right.as_ref().unwrap().count != 3 {
        return;
    }
    if sp.left == sp.right {
        return;
    }
}