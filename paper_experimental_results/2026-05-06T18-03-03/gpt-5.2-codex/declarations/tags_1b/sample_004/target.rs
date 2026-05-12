use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct TNode {
    count: i32,
    left: Option<Rc<RefCell<TNode>>>,
    right: Option<Rc<RefCell<TNode>>>,
}

fn main() {
    let s = Rc::new(RefCell::new(TNode::default()));
    let left_child = Rc::new(RefCell::new(TNode::default()));
    let right_child = Rc::new(RefCell::new(TNode::default()));

    s.borrow_mut().count = 1;
    left_child.borrow_mut().count = 2;
    right_child.borrow_mut().count = 3;

    s.borrow_mut().left = Some(left_child.clone());
    s.borrow_mut().right = Some(right_child.clone());

    let sp = s.clone();

    let left = sp.borrow().left.clone().unwrap();
    if left.borrow().count != 2 {
        std::process::exit(1);
    }

    let right = s.borrow().right.clone().unwrap();
    if right.borrow().count != 3 {
        std::process::exit(2);
    }

    let left_ptr = sp.borrow().left.clone().unwrap();
    let right_ptr = sp.borrow().right.clone().unwrap();
    if Rc::ptr_eq(&left_ptr, &right_ptr) {
        std::process::exit(3);
    }
}