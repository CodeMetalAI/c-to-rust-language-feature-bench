use std::cell::RefCell;
use std::rc::Rc;
use std::process::exit;

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
        sp_borrow
            .left
            .as_ref()
            .unwrap()
            .borrow()
            .count
    };
    if left_count != 2 {
        exit(1);
    }

    let right_count = {
        let s_borrow = s.borrow();
        s_borrow
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .count
    };
    if right_count != 3 {
        exit(2);
    }

    let (left_ptr, right_ptr) = {
        let sp_borrow = sp.borrow();
        (
            Rc::clone(sp_borrow.left.as_ref().unwrap()),
            Rc::clone(sp_borrow.right.as_ref().unwrap()),
        )
    };
    if Rc::ptr_eq(&left_ptr, &right_ptr) {
        exit(3);
    }

    exit(0);
}