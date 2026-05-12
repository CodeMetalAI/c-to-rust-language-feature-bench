use std::rc::Rc;

#[derive(Clone)]
struct Tnode {
    count: i32,
    left: Option<Rc<Tnode>>,
    right: Option<Rc<Tnode>>,
}

fn main() {
    let left_child = Rc::new(Tnode {
        count: 2,
        left: None,
        right: None,
    });

    let right_child = Rc::new(Tnode {
        count: 3,
        left: None,
        right: None,
    });

    let s = Tnode {
        count: 1,
        left: Some(Rc::clone(&left_child)),
        right: Some(Rc::clone(&right_child)),
    };

    let sp = &s;

    if sp.left.as_ref().unwrap().count != 2 {
        std::process::exit(1);
    }

    if s.right.as_ref().unwrap().count != 3 {
        std::process::exit(2);
    }

    if Rc::ptr_eq(&sp.left.as_ref().unwrap(), &sp.right.as_ref().unwrap()) {
        std::process::exit(3);
    }

    std::process::exit(0);
}