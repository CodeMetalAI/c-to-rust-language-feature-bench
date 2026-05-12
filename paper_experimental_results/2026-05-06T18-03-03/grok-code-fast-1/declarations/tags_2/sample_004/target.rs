use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct S1 {
    v1: i32,
    s2p: Option<Rc<RefCell<S2>>>,
}

#[derive(Clone)]
struct S2 {
    v2: i32,
    s1p: Option<Rc<RefCell<S1>>>,
}

fn main() {
    let a = Rc::new(RefCell::new(S1 {
        v1: 0,
        s2p: None,
    }));
    let b = Rc::new(RefCell::new(S2 {
        v2: 0,
        s1p: None,
    }));

    a.borrow_mut().v1 = 10;
    b.borrow_mut().v2 = 20;

    a.borrow_mut().s2p = Some(b.clone());
    b.borrow_mut().s1p = Some(a.clone());

    if a.borrow().s2p.as_ref().unwrap().borrow().v2 != 20 {
        std::process::exit(1);
    }

    if b.borrow().s1p.as_ref().unwrap().borrow().v1 != 10 {
        std::process::exit(2);
    }

    if !Rc::ptr_eq(&a.borrow().s2p.as_ref().unwrap().borrow().s1p.as_ref().unwrap(), &a) {
        std::process::exit(3);
    }

    std::process::exit(0);
}