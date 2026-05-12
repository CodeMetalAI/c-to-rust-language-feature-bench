use std::cell::RefCell;
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
    let a = Rc::new(RefCell::new(S1 { v1: 0, s2p: None }));
    let b = Rc::new(RefCell::new(S2 { v2: 0, s1p: None }));

    {
        let mut a_mut = a.borrow_mut();
        let mut b_mut = b.borrow_mut();
        a_mut.v1 = 10;
        b_mut.v2 = 20;
        a_mut.s2p = Some(b.clone());
        b_mut.s1p = Some(a.clone());
    }

    let v2 = {
        let a_ref = a.borrow();
        let b_rc = a_ref.s2p.as_ref().unwrap().clone();
        let b_ref = b_rc.borrow();
        b_ref.v2
    };
    if v2 != 20 {
        std::process::exit(1);
    }

    let v1 = {
        let b_ref = b.borrow();
        let a_rc = b_ref.s1p.as_ref().unwrap().clone();
        let a_ref = a_rc.borrow();
        a_ref.v1
    };
    if v1 != 10 {
        std::process::exit(2);
    }

    let same = {
        let a_ref = a.borrow();
        let b_rc = a_ref.s2p.as_ref().unwrap().clone();
        drop(a_ref);
        let b_ref = b_rc.borrow();
        let a_from_b = b_ref.s1p.as_ref().unwrap().clone();
        Rc::ptr_eq(&a_from_b, &a)
    };
    if !same {
        std::process::exit(3);
    }
}