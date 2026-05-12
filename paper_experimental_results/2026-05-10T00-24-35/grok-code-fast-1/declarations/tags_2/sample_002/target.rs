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

fn main() -> i32 {
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

    {
        let a_borrow = a.borrow();
        let a_s2 = a_borrow.s2p.as_ref().unwrap().borrow();
        if a_s2.v2 != 20 {
            return 1;
        }
    }

    {
        let b_borrow = b.borrow();
        let b_s1 = b_borrow.s1p.as_ref().unwrap().borrow();
        if b_s1.v1 != 10 {
            return 2;
        }
    }

    {
        let a_borrow = a.borrow();
        let a_s2 = a_borrow.s2p.as_ref().unwrap().borrow();
        let a_s2_s1 = a_s2.s1p.as_ref().unwrap();
        if !Rc::ptr_eq(a_s2_s1, &a) {
            return 3;
        }
    }

    0
}