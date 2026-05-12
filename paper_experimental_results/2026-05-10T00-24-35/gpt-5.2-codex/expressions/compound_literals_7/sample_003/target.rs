use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

struct IntList {
    car: i32,
    cdr: RefCell<Option<Rc<IntList>>>,
}

fn eval(x: &Rc<IntList>, endless: &Rc<IntList>) -> i32 {
    if x.car != 0 {
        return 1;
    }

    let cdr_opt = x.cdr.borrow();
    let cdr = match cdr_opt.as_ref() {
        Some(c) => c,
        None => return 2,
    };

    if !Rc::ptr_eq(cdr, endless) {
        return 2;
    }

    let cdr_cdr_opt = cdr.cdr.borrow();
    let cdr_cdr = match cdr_cdr_opt.as_ref() {
        Some(c) => c,
        None => return 3,
    };

    if !Rc::ptr_eq(cdr_cdr, endless) {
        return 3;
    }

    0
}

fn main() {
    let endless = Rc::new(IntList {
        car: 0,
        cdr: RefCell::new(None),
    });

    *endless.cdr.borrow_mut() = Some(endless.clone());

    let code = eval(&endless, &endless);
    exit(code);
}