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

    let a_s2 = a.borrow().s2p.as_ref().unwrap().clone();
    let a_s2_s1 = a_s2.borrow().s1p.as_ref().unwrap().clone();
    if !Rc::ptr_eq(&a_s2_s1, &a) {
        std::process::exit(3);
    }
}