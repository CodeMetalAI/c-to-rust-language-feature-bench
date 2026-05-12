use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct TNode {
    count: i32,
    left: Option<Rc<RefCell<TNode>>>,
    right: Option<Rc<RefCell<TNode>>>,
}

fn main() {
    let left_child = Rc::new(RefCell::new(TNode {
        count: 2,
        left: None,
        right: None,
    }));

    let right_child = Rc::new(RefCell::new(TNode {
        count: 3,
        left: None,
        right: None,
    }));

    let s = Rc::new(RefCell::new(TNode {
        count: 1,
        left: Some(Rc::clone(&left_child)),
        right: Some(Rc::clone(&right_child)),
    }));

    let sp = Rc::clone(&s);

    if sp.borrow().left.as_ref().unwrap().borrow().count != 2 {
        std::process::exit(1);
    }

    if s.borrow().right.as_ref().unwrap().borrow().count != 3 {
        std::process::exit(2);
    }

    if Rc::ptr_eq(sp.borrow().left.as_ref().unwrap(), sp.borrow().right.as_ref().unwrap()) {
        std::process::exit(3);
    }

    std::process::exit(0);
}