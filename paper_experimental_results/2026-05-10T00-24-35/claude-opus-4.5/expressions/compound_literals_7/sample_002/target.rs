use std::rc::Rc;
use std::cell::RefCell;

struct IntList {
    car: i32,
    cdr: Option<Rc<RefCell<IntList>>>,
}

fn main() {
    // Create the self-referential structure
    let endless_zeros = Rc::new(RefCell::new(IntList {
        car: 0,
        cdr: None,
    }));
    
    // Make it self-referential
    endless_zeros.borrow_mut().cdr = Some(Rc::clone(&endless_zeros));
    
    let exit_code = eval(&endless_zeros, &endless_zeros);
    std::process::exit(exit_code);
}

fn eval(x: &Rc<RefCell<IntList>>, endless_zeros: &Rc<RefCell<IntList>>) -> i32 {
    let x_borrowed = x.borrow();
    
    if x_borrowed.car != 0 {
        return 1;
    }
    
    // Check if x.cdr points to endless_zeros
    match &x_borrowed.cdr {
        Some(cdr) => {
            if !Rc::ptr_eq(cdr, endless_zeros) {
                return 2;
            }
            // Check if x.cdr->cdr points to endless_zeros
            let cdr_borrowed = cdr.borrow();
            match &cdr_borrowed.cdr {
                Some(cdr_cdr) => {
                    if !Rc::ptr_eq(cdr_cdr, endless_zeros) {
                        return 3;
                    }
                }
                None => return 3,
            }
        }
        None => return 2,
    }
    
    0
}