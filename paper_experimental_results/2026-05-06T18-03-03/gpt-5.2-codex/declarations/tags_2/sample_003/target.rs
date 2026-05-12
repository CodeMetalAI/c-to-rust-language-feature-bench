use std::cell::RefCell;
use std::rc::Rc;
use std::process::exit;

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

    let v2 = a.borrow().s2p.as_ref().unwrap().borrow().v2;
    if v2 != 20 {
        exit(1);
    }

    let v1 = b.borrow().s1p.as_ref().unwrap().borrow().v1;
    if v1 != 10 {
        exit(2);
    }

    let s1p_rc = {
        let s2p = a.borrow().s2p.as_ref().unwrap().clone();
        s2p.borrow().s1p.as_ref().unwrap().clone()
    };

    if !Rc::ptr_eq(&s1p_rc, &a) {
        exit(3);
    }
}