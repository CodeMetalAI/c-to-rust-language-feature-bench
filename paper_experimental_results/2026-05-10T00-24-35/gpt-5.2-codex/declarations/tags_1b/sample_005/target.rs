use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
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
        left: Some(left_child.clone()),
        right: Some(right_child.clone()),
    }));

    let sp = s.clone();

    let left_count = {
        let sp_borrow = sp.borrow();
        let left = sp_borrow.left.as_ref().unwrap();
        left.borrow().count
    };
    if left_count != 2 {
        std::process::exit(1);
    }

    let right_count = {
        let s_borrow = s.borrow();
        let right = s_borrow.right.as_ref().unwrap();
        right.borrow().count
    };
    if right_count != 3 {
        std::process::exit(2);
    }

    let same_ptr = {
        let sp_borrow = sp.borrow();
        let left = sp_borrow.left.as_ref().unwrap();
        let right = sp_borrow.right.as_ref().unwrap();
        Rc::ptr_eq(left, right)
    };
    if same_ptr {
        std::process::exit(3);
    }

    std::process::exit(0);
}