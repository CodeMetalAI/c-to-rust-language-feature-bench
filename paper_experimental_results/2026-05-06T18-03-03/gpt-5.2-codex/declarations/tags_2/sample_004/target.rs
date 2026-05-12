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
    let a = Rc::new(RefCell::new(S1 { v1: 10, s2p: None }));
    let b = Rc::new(RefCell::new(S2 { v2: 20, s1p: None }));

    a.borrow_mut().s2p = Some(b.clone());
    b.borrow_mut().s1p = Some(a.clone());

    {
        let a_borrow = a.borrow();
        let s2 = a_borrow.s2p.as_ref().unwrap();
        if s2.borrow().v2 != 20 {
            std::process::exit(1);
        }
    }

    {
        let b_borrow = b.borrow();
        let s1 = b_borrow.s1p.as_ref().unwrap();
        if s1.borrow().v1 != 10 {
            std::process::exit(2);
        }
    }

    {
        let a_borrow = a.borrow();
        let s2 = a_borrow.s2p.as_ref().unwrap();
        let s1 = s2.borrow().s1p.as_ref().unwrap().clone();
        if !Rc::ptr_eq(&s1, &a) {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}