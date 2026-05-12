use std::cell::RefCell;
use std::rc::Rc;

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

    let left_count = {
        let sp_borrow = sp.borrow();
        let left_rc = sp_borrow.left.as_ref().unwrap();
        left_rc.borrow().count
    };
    if left_count != 2 {
        std::process::exit(1);
    }

    let right_count = {
        let s_borrow = s.borrow();
        let right_rc = s_borrow.right.as_ref().unwrap();
        right_rc.borrow().count
    };
    if right_count != 3 {
        std::process::exit(2);
    }

    let same_ptr = {
        let sp_borrow = sp.borrow();
        let left_rc = sp_borrow.left.as_ref().unwrap();
        let right_rc = sp_borrow.right.as_ref().unwrap();
        Rc::ptr_eq(left_rc, right_rc)
    };
    if same_ptr {
        std::process::exit(3);
    }

    std::process::exit(0);
}