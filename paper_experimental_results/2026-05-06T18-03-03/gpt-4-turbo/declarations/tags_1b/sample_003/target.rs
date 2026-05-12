struct Tnode {
    count: i32,
    left: Option<Box<Tnode>>,
    right: Option<Box<Tnode>>,
}

fn main() {
    let mut left_child = Tnode {
        count: 2,
        left: None,
        right: None,
    };

    let mut right_child = Tnode {
        count: 3,
        left: None,
        right: None,
    };

    let mut s = Tnode {
        count: 1,
        left: Some(Box::new(left_child)),
        right: Some(Box::new(right_child)),
    };

    let sp = &s;

    if sp.left.as_ref().unwrap().count != 2 {
        std::process::exit(1);
    }

    if s.right.as_ref().unwrap().count != 3 {
        std::process::exit(2);
    }

    if std::ptr::eq(sp.left.as_ref().unwrap(), sp.right.as_ref().unwrap()) {
        std::process::exit(3);
    }

    std::process::exit(0);
}