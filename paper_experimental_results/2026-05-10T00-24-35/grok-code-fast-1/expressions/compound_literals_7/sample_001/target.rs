use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Clone)]
struct IntList {
    car: i32,
    cdr: Weak<RefCell<IntList>>,
}

fn eval(x: Rc<RefCell<IntList>>, endless_zeros: &Rc<RefCell<IntList>>) -> i32 {
    let x_ref = x.borrow();
    if x_ref.car != 0 {
        return 1;
    }
    let cdr_rc = x_ref.cdr.upgrade().unwrap();
    if !Rc::ptr_eq(&cdr_rc, endless_zeros) {
        return 2;
    }
    let cdr_ref = cdr_rc.borrow();
    let cdr_cdr_rc = cdr_ref.cdr.upgrade().unwrap();
    if !Rc::ptr_eq(&cdr_cdr_rc, endless_zeros) {
        return 3;
    }
    0
}

fn main() {
    let endless_zeros = Rc::new_cyclic(|weak| {
        RefCell::new(IntList {
            car: 0,
            cdr: weak.clone(),
        })
    });
    let result = eval(endless_zeros.clone(), &endless_zeros);
    std::process::exit(result);
}