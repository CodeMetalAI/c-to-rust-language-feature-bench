struct S1 {
    v1: i32,
    s2p: Option<std::cell::RefCell<std::rc::Rc<S2>>>,
}

struct S2 {
    v2: i32,
    s1p: Option<std::cell::RefCell<std::rc::Rc<S1>>>,
}

fn main() {
    let a = std::rc::Rc::new(std::cell::RefCell::new(S1 { v1: 10, s2p: None }));
    let b = std::rc::Rc::new(std::cell::RefCell::new(S2 { v2: 20, s1p: None }));

    a.borrow_mut().s2p = Some(std::cell::RefCell::new(b.clone()));
    b.borrow_mut().s1p = Some(std::cell::RefCell::new(a.clone()));

    if b.borrow().v2 != 20 {
        std::process::exit(1);
    }

    if a.borrow().v1 != 10 {
        std::process::exit(2);
    }

    if !std::ptr::eq(&*a.borrow(), &*b.borrow().s1p.as_ref().unwrap().borrow()) {
        std::process::exit(3);
    }

    std::process::exit(0);
}