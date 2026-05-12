use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

struct S1 {
    v1: i32,
    s2p: Option<Rc<RefCell<S2>>>,
}

struct S2 {
    v2: i32,
    s1p: Option<Rc<RefCell<S1>>>,
}

fn main() {
    let a = Rc::new(RefCell::new(S1 { v1: 10, s2p: None }));
    let b = Rc::new(RefCell::new(S2 { v2: 20, s1p: None }));

    {
        let mut a_mut = a.borrow_mut();
        a_mut.s2p = Some(Rc::clone(&b));
    }
    {
        let mut b_mut = b.borrow_mut();
        b_mut.s1p = Some(Rc::clone(&a));
    }

    let v2 = {
        let s2_rc = a.borrow().s2p.as_ref().unwrap().clone();
        s2_rc.borrow().v2
    };
    if v2 != 20 {
        exit(1);
    }

    let v1 = {
        let s1_rc = b.borrow().s1p.as_ref().unwrap().clone();
        s1_rc.borrow().v1
    };
    if v1 != 10 {
        exit(2);
    }

    let same_ptr = {
        let s2_rc = a.borrow().s2p.as_ref().unwrap().clone();
        let s1_rc = s2_rc.borrow().s1p.as_ref().unwrap().clone();
        Rc::ptr_eq(&a, &s1_rc)
    };
    if !same_ptr {
        exit(3);
    }

    exit(0);
}