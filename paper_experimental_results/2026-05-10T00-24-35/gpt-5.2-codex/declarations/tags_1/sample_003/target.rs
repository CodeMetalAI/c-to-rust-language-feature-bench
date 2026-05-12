use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

#[derive(Default)]
struct Tnode {
    count: i32,
    left: Option<Rc<RefCell<Tnode>>>,
    right: Option<Rc<RefCell<Tnode>>>,
}

fn main() {
    let s = Rc::new(RefCell::new(Tnode::default()));
    let left_child = Rc::new(RefCell::new(Tnode::default()));
    let right_child = Rc::new(RefCell::new(Tnode::default()));

    s.borrow_mut().count = 1;
    left_child.borrow_mut().count = 2;
    right_child.borrow_mut().count = 3;

    s.borrow_mut().left = Some(left_child.clone());
    s.borrow_mut().right = Some(right_child.clone());

    let sp = s.clone();

    if sp.borrow().left.as_ref().unwrap().borrow().count != 2 {
        exit(1);
    }

    if s.borrow().right.as_ref().unwrap().borrow().count != 3 {
        exit(2);
    }

    let left = sp.borrow().left.as_ref().unwrap().clone();
    let right = sp.borrow().right.as_ref().unwrap().clone();
    if Rc::ptr_eq(&left, &right) {
        exit(3);
    }

    exit(0);
}