use lazy_static::lazy_static;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct IntList {
    car: i32,
    cdr: Rc<IntList>,
}

lazy_static! {
    static ref ENDLESS_ZEROS: Rc<IntList> = Rc::new_cyclic(|weak| IntList {
        car: 0,
        cdr: weak.upgrade().unwrap(),
    });
}

fn eval(x: Rc<IntList>) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if !Rc::ptr_eq(&x.cdr, &ENDLESS_ZEROS) {
        return 2;
    }
    if !Rc::ptr_eq(&x.cdr.cdr, &ENDLESS_ZEROS) {
        return 3;
    }
    0
}

fn main() {
    std::process::exit(eval(ENDLESS_ZEROS.clone()));
}