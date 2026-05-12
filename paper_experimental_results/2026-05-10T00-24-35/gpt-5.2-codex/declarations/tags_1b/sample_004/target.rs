use std::cell::RefCell;
use std::process::exit;
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

    s.borrow_mut().left = Some(Rc::clone(&left_child));
    s.borrow_mut().right = Some(Rc::clone(&right_child));

    let sp = Rc::clone(&s);

    let left_count = sp
        .borrow()
        .left
        .as_ref()
        .unwrap()
        .borrow()
        .count;
    if left_count != 2 {
        exit(1);
    }

    let right_count = s
        .borrow()
        .right
        .as_ref()
        .unwrap()
        .borrow()
        .count;
    if right_count != 3 {
        exit(2);
    }

    let left_rc = sp.borrow().left.as_ref().unwrap().clone();
    let right_rc = sp.borrow().right.as_ref().unwrap().clone();
    if Rc::ptr_eq(&left_rc, &right_rc) {
        exit(3);
    }

    exit(0);
}