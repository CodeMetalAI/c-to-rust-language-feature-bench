use std::rc::Rc;
use std::cell::RefCell;
use lazy_static::lazy_static;

#[derive(Clone)]
struct IntList {
    car: i32,
    cdr: Rc<RefCell<IntList>>,
}

lazy_static! {
    static ref ENDLESS_ZEROS: Rc<RefCell<IntList>> = Rc::new_cyclic(|weak| {
        RefCell::new(IntList {
            car: 0,
            cdr: weak.upgrade().unwrap(),
        })
    });
}

fn eval(x: IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if !Rc::ptr_eq(&x.cdr, &*ENDLESS_ZEROS) {
        return 2;
    }
    {
        let cdr = x.cdr.borrow();
        if !Rc::ptr_eq(&cdr.cdr, &*ENDLESS_ZEROS) {
            return 3;
        }
    }
    0
}

fn main() {
    let exit_code = eval(ENDLESS_ZEROS.borrow().clone());
    std::process::exit(exit_code);
}