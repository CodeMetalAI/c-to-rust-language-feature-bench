use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

fn main() {
    let i1 = Rc::new(RefCell::new(1));
    let i2 = Rc::new(RefCell::new(2));
    let i3 = Rc::new(RefCell::new(3));
    let i4 = Rc::new(RefCell::new(0));
    let i5 = Rc::new(RefCell::new(0));

    let p_i1 = Rc::clone(&i1);
    let p_i2 = Rc::clone(&i2);
    let p_i4 = Rc::clone(&i4);
    let p_i5 = Rc::clone(&i5);

    if *i1.borrow() != 1 {
        exit(1);
    }
    if *i2.borrow() != 2 {
        exit(2);
    }
    if *i3.borrow() != 3 {
        exit(3);
    }

    if *i4.borrow() != 0 {
        exit(4);
    }
    if *i5.borrow() != 0 {
        exit(5);
    }

    if !Rc::ptr_eq(&p_i1, &i1) {
        exit(6);
    }
    if !Rc::ptr_eq(&p_i2, &i2) {
        exit(7);
    }
    if !Rc::ptr_eq(&p_i4, &i4) {
        exit(8);
    }
    if !Rc::ptr_eq(&p_i5, &i5) {
        exit(9);
    }

    *i1.borrow_mut() = 10;
    *i2.borrow_mut() = 20;
    *i4.borrow_mut() = 30;
    *i5.borrow_mut() = 40;

    if *p_i1.borrow() != 10 {
        exit(10);
    }
    if *p_i2.borrow() != 20 {
        exit(11);
    }
    if *p_i4.borrow() != 30 {
        exit(12);
    }
    if *p_i5.borrow() != 40 {
        exit(13);
    }

    exit(0);
}