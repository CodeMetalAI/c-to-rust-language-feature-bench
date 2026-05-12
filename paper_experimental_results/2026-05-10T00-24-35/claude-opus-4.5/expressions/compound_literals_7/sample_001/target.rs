use std::cell::RefCell;
use std::rc::Rc;

struct IntList {
    car: i32,
    cdr: Option<Rc<RefCell<IntList>>>,
}

fn eval(x: &Rc<RefCell<IntList>>, endless_zeros: &Rc<RefCell<IntList>>) -> i32 {
    let x_borrowed = x.borrow();
    
    if x_borrowed.car != 0 {
        return 1;
    }
    
    let cdr = match &x_borrowed.cdr {
        Some(c) => c,
        None => return 2,
    };
    
    if !Rc::ptr_eq(cdr, endless_zeros) {
        return 2;
    }
    
    let cdr_borrowed = cdr.borrow();
    let cdr_cdr = match &cdr_borrowed.cdr {
        Some(c) => c,
        None => return 3,
    };
    
    if !Rc::ptr_eq(cdr_cdr, endless_zeros) {
        return 3;
    }
    
    0
}

fn main() {
    // Create the self-referential structure
    let endless_zeros = Rc::new(RefCell::new(IntList {
        car: 0,
        cdr: None,
    }));
    
    // Make it self-referential
    endless_zeros.borrow_mut().cdr = Some(Rc::clone(&endless_zeros));
    
    let result = eval(&endless_zeros, &endless_zeros);
    std::process::exit(result);
}