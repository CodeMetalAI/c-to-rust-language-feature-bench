use std::cell::RefCell;
use std::rc::Rc;

struct IntList {
    car: i32,
    cdr: Option<Rc<RefCell<IntList>>>,
}

fn eval(x: Rc<RefCell<IntList>>) -> i32 {
    if x.borrow().car != 0 {
        return 1;
    }
    let cdr = x.borrow().cdr.clone().unwrap();
    if !Rc::ptr_eq(&cdr, &x) {
        return 2;
    }
    let cdr2 = cdr.borrow().cdr.clone().unwrap();
    if !Rc::ptr_eq(&cdr2, &x) {
        return 3;
    }
    0
}

fn main() {
    let node = Rc::new(RefCell::new(IntList { car: 0, cdr: None }));
    node.borrow_mut().cdr = Some(Rc::clone(&node));
    let result = eval(Rc::clone(&node));
    std::process::exit(result);
}