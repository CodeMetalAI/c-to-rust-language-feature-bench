#[macro_use]
extern crate lazy_static;

use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
struct IntList {
    car: i32,
    cdr: Weak<IntList>,
}

lazy_static! {
    static ref ENDLESS_ZEROS: Rc<IntList> = Rc::new_cyclic(|weak| IntList {
        car: 0,
        cdr: weak.clone(),
    });
}

fn eval(x: &Rc<IntList>) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if x.cdr.upgrade().as_ref() != Some(x) {
        return 2;
    }
    if x.cdr.upgrade().unwrap().cdr.upgrade().as_ref() != Some(x) {
        return 3;
    }
    0
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}