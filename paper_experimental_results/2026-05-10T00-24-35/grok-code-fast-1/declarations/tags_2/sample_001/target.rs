use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct S1 {
    v1: i32,
    s2p: Option<Rc<RefCell<S2>>>,
}

#[derive(Debug)]
struct S2 {
    v2: i32,
    s1p: Option<Rc<RefCell<S1>>>,
}

fn main() -> i32 {
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
        b_mut.s1p = Some(Rc::clone(&a));
    }

    // Check a.s2p->v2 != 20
    let a_ref = a.borrow();
    if a_ref.s2p.as_ref().unwrap().borrow().v2 != 20 {
        return 1;
    }

    // Check b.s1p->v1 != 10
    let b_ref = b.borrow();
    if b_ref.s1p.as_ref().unwrap().borrow().v1 != 10 {
        return 2;
    }

    // Check a.s2p->s1p != &a
    if !Rc::ptr_eq(&a, &a_ref.s2p.as_ref().unwrap().borrow().s1p.as_ref().unwrap()) {
        return 3;
    }

    return 0;
}