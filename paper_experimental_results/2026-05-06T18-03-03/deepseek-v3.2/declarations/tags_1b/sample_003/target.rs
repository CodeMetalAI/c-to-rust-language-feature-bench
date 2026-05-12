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
    let s = TNODE {
        count: 1,
        left: Some(Box::new(left_child)),
        right: Some(Box::new(right_child)),
    };
    let sp = &s;

    if sp.left.as_ref().unwrap().count != 2 {
        return std::process::exit(1);
    }
    if s.right.as_ref().unwrap().count != 3 {
        return std::process::exit(2);
    }
    if sp.left == sp.right {
        return std::process::exit(3);
    }
    std::process::exit(0);
}