use std::cell::RefCell;
use std::rc::Rc;

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

fn main() {
    let a = Rc::new(RefCell::new(S1 { v1: 0, s2p: None }));
    let b = Rc::new(RefCell::new(S2 { v2: 0, s1p: None }));

    {
        let mut a_ref = a.borrow_mut();
        a_ref.v1 = 10;
        a_ref.s2p = Some(Rc::clone(&b));
    }

    {
        let mut b_ref = b.borrow_mut();
        b_ref.v2 = 20;
        b_ref.s1p = Some(Rc::clone(&a));
    }

    if a.borrow().s2p.as_ref().unwrap().borrow().v2 != 20 {
        std::process::exit(1);
    }

    if b.borrow().s1p.as_ref().unwrap().borrow().v1 != 10 {
        std::process::exit(2);
    }

    if a.borrow().s2p.as_ref().unwrap().borrow().s1p.as_ref().unwrap() != &a {
        std::process::exit(3);
    }

    std::process::exit(0);
}