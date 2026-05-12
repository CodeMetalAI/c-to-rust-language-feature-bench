use std::cell::RefCell;
use std::rc::Rc;

struct IntList {
    car: i32,
    cdr: RefCell<Option<Rc<IntList>>>,
}

fn eval(x: &Rc<IntList>, endless_zeros: &Rc<IntList>) -> i32 {
    if x.car != 0 {
        return 1;
    }
    let cdr_ref = x.cdr.borrow();
    if let Some(ref cdr) = *cdr_ref {
        if !Rc::ptr_eq(cdr, endless_zeros) {
            return 2;
        }
        let cdr_cdr_ref = cdr.cdr.borrow();
        if let Some(ref cdr_cdr) = *cdr_cdr_ref {
            if !Rc::ptr_eq(cdr_cdr, endless_zeros) {
                return 3;
            }
        } else {
            return 3;
        }
    } else {
        return 2;
    }
    0
}

fn main() {
    let endless_zeros = Rc::new(IntList {
        car: 0,
        cdr: RefCell::new(None),
    });
    
    // Make it self-referential
    *endless_zeros.cdr.borrow_mut() = Some(Rc::clone(&endless_zeros));
    
    let result = eval(&endless_zeros, &endless_zeros);
    std::process::exit(result);
}