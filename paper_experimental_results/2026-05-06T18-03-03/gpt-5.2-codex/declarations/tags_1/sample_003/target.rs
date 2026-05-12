use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

#[derive(Debug)]
struct TNode {
    count: i32,
    left: Option<Rc<RefCell<TNode>>>,
    right: Option<Rc<RefCell<TNode>>>,
}

fn main() {
    let s = Rc::new(RefCell::new(TNode {
        count: 1,
        left: None,
        right: None,
    }));
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

    {
        let mut s_mut = s.borrow_mut();
        s_mut.left = Some(left_child.clone());
        s_mut.right = Some(right_child.clone());
    }

    let sp = s.clone();

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