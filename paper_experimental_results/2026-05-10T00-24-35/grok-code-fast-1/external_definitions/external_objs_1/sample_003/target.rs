use std::cell::RefCell;
use std::process;
use std::rc::Rc;

lazy_static::lazy_static! {
    static ref i1: Rc<RefCell<i32>> = Rc::new(RefCell::new(1));
    static ref i2: Rc<RefCell<i32>> = Rc::new(RefCell::new(2));
    static ref i3: Rc<RefCell<i32>> = Rc::new(RefCell::new(3));
    static ref i4: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));
    static ref i5: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));
    static ref p_i1: Rc<RefCell<i32>> = i1.clone();
    static ref p_i2: Rc<RefCell<i32>> = i2.clone();
    static ref p_i4: Rc<RefCell<i32>> = i4.clone();
    static ref p_i5: Rc<RefCell<i32>> = i5.clone();
}

fn main() {
    if *i1.borrow() != 1 {
        process::exit(1);
    }
    if *i2.borrow() != 2 {
        process::exit(2);
    }
    if *i3.borrow() != 3 {
        process::exit(3);
    }
    if *i4.borrow() != 0 {
        process::exit(4);
    }
    if *i5.borrow() != 0 {
        process::exit(5);
    }
    if !Rc::ptr_eq(&p_i1, &i1) {
        process::exit(6);
    }
    if !Rc::ptr_eq(&p_i2, &i2) {
        process::exit(7);
    }
    if !Rc::ptr_eq(&p_i4, &i4) {
        process::exit(8);
    }
    if !Rc::ptr_eq(&p_i5, &i5) {
        process::exit(9);
    }
    *i1.borrow_mut() = 10;
    *i2.borrow_mut() = 20;
    *i4.borrow_mut() = 30;
    *i5.borrow_mut() = 40;
    if *p_i1.borrow() != 10 {
        process::exit(10);
    }
    if *p_i2.borrow() != 20 {
        process::exit(11);
    }
    if *p_i4.borrow() != 30 {
        process::exit(12);
    }
    if *p_i5.borrow() != 40 {
        process::exit(13);
    }
    process::exit(0);
}