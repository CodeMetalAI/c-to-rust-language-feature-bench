use std::rc::Rc;
use std::sync::OnceLock;

struct IntList {
    car: i32,
    cdr: Rc<IntList>,
}

static ENDLESS: OnceLock<Rc<IntList>> = OnceLock::new();

fn endless_zeros() -> &'static Rc<IntList> {
    ENDLESS.get_or_init(|| Rc::new_cyclic(|weak| IntList {
        car: 0,
        cdr: weak.upgrade().unwrap(),
    }))
}

fn eval(x: Rc<IntList>) -> i32 {
    if x.car != 0 {
        return 1;
    }
    let cdr = &x.cdr;
    if !Rc::ptr_eq(cdr, endless_zeros()) {
        return 2;
    }
    let cdr2 = &cdr.cdr;
    if !Rc::ptr_eq(cdr2, endless_zeros()) {
        return 3;
    }
    0
}

fn main() {
    std::process::exit(eval(endless_zeros().clone()));
}