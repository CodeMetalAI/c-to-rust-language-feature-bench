use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct Tnode {
    count: i32,
    left: Option<Rc<RefCell<Tnode>>>,
    right: Option<Rc<RefCell<Tnode>>>,
}

fn main() {
    let left_child = Rc::new(RefCell::new(Tnode {
        count: 2,
        left: None,
        right: None,
    }));
    let right_child = Rc::new(RefCell::new(Tnode {
        count: 3,
        left: None,
        right: None,
    }));
    let s = Rc::new(RefCell::new(Tnode {
        count: 1,
        left: None,
        right: None,
    }));
    s.borrow_mut().left = Some(left_child.clone());
    s.borrow_mut().right = Some(right_child.clone());
    let sp = s.clone();

    if sp.borrow().left.as_ref().unwrap().borrow().count != 2 {
        std::process::exit(1);
    }
    if s.borrow().right.as_ref().unwrap().borrow().count != 3 {
        std::process::exit(2);
    }
    if Rc::ptr_eq(
        sp.borrow().left.as_ref().unwrap(),
        sp.borrow().right.as_ref().unwrap(),
    ) {
        std::process::exit(3);
    }
    std::process::exit(0);
}