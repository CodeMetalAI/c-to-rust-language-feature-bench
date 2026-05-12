use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct IntList {
    car: i32,
    cdr: Rc<RefCell<IntList>>,
}

fn eval(x: IntList, endless: &Rc<RefCell<IntList>>) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if !Rc::ptr_eq(&x.cdr, endless) {
        return 2;
    }
    if !Rc::ptr_eq(&x.cdr.borrow().cdr, endless) {
        return 3;
    }
    0
}

fn main() {
    let endless_zeros = Rc::new_cyclic(|weak| {
        RefCell::new(IntList {
            car: 0,
            cdr: weak.upgrade().unwrap(),
        })
    });
    let x = endless_zeros.borrow().clone();
    std::process::exit(eval(x, &endless_zeros));
}