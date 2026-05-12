struct Tnode {
    count: i32,
    left: Option<Box<Tnode>>,
    right: Option<Box<Tnode>>,
}

fn main() {
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

    let s = Tnode {
        count: 1,
        left: Some(Box::new(left_child)),
        right: Some(Box::new(right_child)),
    };

    if s.left.as_ref().unwrap().count != 2 {
        std::process::exit(1);
    }

    if s.right.as_ref().unwrap().count != 3 {
        std::process::exit(2);
    }

    if s.left.as_ref().map(|b| b.as_ref() as *const Tnode) 
        == s.right.as_ref().map(|b| b.as_ref() as *const Tnode) {
        std::process::exit(3);
    }

    std::process::exit(0);
}