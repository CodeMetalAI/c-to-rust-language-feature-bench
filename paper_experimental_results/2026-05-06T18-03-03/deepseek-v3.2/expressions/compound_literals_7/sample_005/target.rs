use std::rc::Rc;

struct IntList {
    car: i32,
    cdr: Rc<IntList>,
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: unsafe { std::mem::transmute(&ENDLESS_ZEROS) },
};

fn eval(x: &IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if !Rc::ptr_eq(&x.cdr, &ENDLESS_ZEROS.cdr) {
        return 2;
    }
    if !Rc::ptr_eq(&x.cdr.cdr, &ENDLESS_ZEROS.cdr) {
        return 3;
    }
    0
}

fn main() {
    std::process::exit(eval(&ENDLESS_ZEROS));
}