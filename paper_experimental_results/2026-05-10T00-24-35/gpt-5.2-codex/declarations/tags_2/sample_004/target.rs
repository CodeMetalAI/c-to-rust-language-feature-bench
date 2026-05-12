use std::cell::RefCell;
use std::process::exit;
use std::rc::{Rc, Weak};

struct S1 {
    v1: i32,
    s2p: Option<Weak<RefCell<S2>>>,
}

struct S2 {
    v2: i32,
    s1p: Option<Weak<RefCell<S1>>>,
}

fn main() {
    let a = Rc::new(RefCell::new(S1 { v1: 0, s2p: None }));
    let b = Rc::new(RefCell::new(S2 { v2: 0, s1p: None }));

    {
        let mut a_mut = a.borrow_mut();
        a_mut.v1 = 10;
        a_mut.s2p = Some(Rc::downgrade(&b));
    }

    {
        let mut b_mut = b.borrow_mut();
        b_mut.v2 = 20;
        b_mut.s1p = Some(Rc::downgrade(&a));
    }

    let v2 = a
        .borrow()
        .s2p
        .as_ref()
        .and_then(|w| w.upgrade())
        .map(|rc| rc.borrow().v2)
        .unwrap();
    if v2 != 20 {
        exit(1);
    }

    let v1 = b
        .borrow()
        .s1p
        .as_ref()
        .and_then(|w| w.upgrade())
        .map(|rc| rc.borrow().v1)
        .unwrap();
    if v1 != 10 {
        exit(2);
    }

    let s1p = a
        .borrow()
        .s2p
        .as_ref()
        .and_then(|w| w.upgrade())
        .and_then(|rc| rc.borrow().s1p.as_ref().and_then(|w2| w2.upgrade()))
        .unwrap();
    if !Rc::ptr_eq(&s1p, &a) {
        exit(3);
    }

    exit(0);
}