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
    let a = Rc::new(RefCell::new(S1 { v1: 0, s2p: None }));
    let b = Rc::new(RefCell::new(S2 { v2: 0, s1p: None }));

    {
        let mut a_mut = a.borrow_mut();
        a_mut.v1 = 10;
        a_mut.s2p = Some(Rc::clone(&b));
    }
    {
        let mut b_mut = b.borrow_mut();
        b_mut.v2 = 20;
        b_mut.s1p = Some(Rc::downgrade(&a));
    }

    let v2 = {
        let a_ref = a.borrow();
        let s2p = a_ref.s2p.as_ref().unwrap();
        s2p.borrow().v2
    };
    if v2 != 20 {
        std::process::exit(1);
    }

    let v1 = {
        let b_ref = b.borrow();
        let s1p = b_ref.s1p.as_ref().unwrap().upgrade().unwrap();
        s1p.borrow().v1
    };
    if v1 != 10 {
        std::process::exit(2);
    }

    let same = {
        let a_ref = a.borrow();
        let s2p = a_ref.s2p.as_ref().unwrap();
        let b_ref = s2p.borrow();
        let s1p_rc = b_ref.s1p.as_ref().unwrap().upgrade().unwrap();
        Rc::ptr_eq(&s1p_rc, &a)
    };
    if !same {
        std::process::exit(3);
    }

    std::process::exit(0);
}