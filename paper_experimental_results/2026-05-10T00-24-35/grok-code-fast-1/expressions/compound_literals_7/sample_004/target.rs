use std::rc::Rc;
use std::sync::OnceLock;

#[derive(Debug)]
struct IntList {
    car: i32,
    cdr: Rc<IntList>,
}

static ENDLESS_ZEROS: OnceLock<Rc<IntList>> = OnceLock::new();

fn eval(x: Rc<IntList>) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if !Rc::ptr_eq(&x.cdr, ENDLESS_ZEROS.get().unwrap()) {
        return 2;
    }
    if !Rc::ptr_eq(&x.cdr.cdr, ENDLESS_ZEROS.get().unwrap()) {
        return 3;
    }
    0
}

fn main() {
    let endless_zeros = ENDLESS_ZEROS.get_or_init(|| {
        Rc::new_cyclic(|weak| IntList {
            car: 0,
            cdr: weak.upgrade().unwrap(),
        })
    });
    let result = eval(Rc::clone(endless_zeros));
    std::process::exit(result);
}