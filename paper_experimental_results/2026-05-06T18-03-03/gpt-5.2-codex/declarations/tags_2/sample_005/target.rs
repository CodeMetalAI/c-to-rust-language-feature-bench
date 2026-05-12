use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct S1 {
    v1: i32,
    s2p: Option<Rc<RefCell<S2>>>,
}

struct S2 {
    v2: i32,
    s1p: Option<Weak<RefCell<S1>>>,
}

fn main() {
    let a = Rc::new(RefCell::new(S1 { v1: 10, s2p: None }));
    let b = Rc::new(RefCell::new(S2 { v2: 20, s1p: None }));

    a.borrow_mut().s2p = Some(Rc::clone(&b));
    b.borrow_mut().s1p = Some(Rc::downgrade(&a));

    let v2 = a
        .borrow()
        .s2p
        .as_ref()
        .unwrap()
        .borrow()
        .v2;
    if v2 != 20 {
        std::process::exit(1);
    }

    let v1 = b
        .borrow()
        .s1p
        .as_ref()
        .and_then(|w| w.upgrade())
        .unwrap()
        .borrow()
        .v1;
    if v1 != 10 {
        std::process::exit(2);
    }

    let s1_from_b = b
        .borrow()
        .s1p
        .as_ref()
        .and_then(|w| w.upgrade())
        .unwrap();
    if !Rc::ptr_eq(&s1_from_b, &a) {
        std::process::exit(3);
    }
}