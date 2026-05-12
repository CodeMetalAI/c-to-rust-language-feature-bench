struct Tnode {
    count: i32,
    left: Option<Box<Tnode>>,
    right: Option<Box<Tnode>>,
}

fn main() {
    let mut s = Tnode {
        count: 1,
        left: Some(Box::new(Tnode {
            count: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(Tnode {
            count: 3,
            left: None,
            right: None,
        })),
    };

    if s.left.as_ref().unwrap().count!= 2 {
        std::process::exit(1);
    }

    if s.right.as_ref().unwrap().count!= 3 {
        std::process::exit(2);
    }

    if s.left.as_ref().unwrap() as *const _ == s.right.as_ref().unwrap() as *const _ {
        std::process::exit(3);
    }
}