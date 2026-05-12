use std::cell::RefCell;
use lazy_static::lazy_static;

lazy_static! {
    static ref I1: RefCell<i32> = RefCell::new(1);
    static ref I2: RefCell<i32> = RefCell::new(2);
    static ref I3: RefCell<i32> = RefCell::new(3);
    static ref I4: RefCell<i32> = RefCell::new(0);
    static ref I5: RefCell<i32> = RefCell::new(0);
    static ref P_I1: &'static RefCell<i32> = &I1;
    static ref P_I2: &'static RefCell<i32> = &I2;
    static ref P_I4: &'static RefCell<i32> = &I4;
    static ref P_I5: &'static RefCell<i32> = &I5;
}

fn main() {
    if *I1.borrow() != 1 {
        std::process::exit(1);
    }
    if *I2.borrow() != 2 {
        std::process::exit(2);
    }
    if *I3.borrow() != 3 {
        std::process::exit(3);
    }
    if *I4.borrow() != 0 {
        std::process::exit(4);
    }
    if *I5.borrow() != 0 {
        std::process::exit(5);
    }
    if *P_I1 != *(&I1) {
        std::process::exit(6);
    }
    if *P_I2 != *(&I2) {
        std::process::exit(7);
    }
    if *P_I4 != *(&I4) {
        std::process::exit(8);
    }
    if *P_I5 != *(&I5) {
        std::process::exit(9);
    }
    *I1.borrow_mut() = 10;
    *I2.borrow_mut() = 20;
    *I4.borrow_mut() = 30;
    *I5.borrow_mut() = 40;
    if *P_I1.borrow() != 10 {
        std::process::exit(10);
    }
    if *P_I2.borrow() != 20 {
        std::process::exit(11);
    }
    if *P_I4.borrow() != 30 {
        std::process::exit(12);
    }
    if *P_I5.borrow() != 40 {
        std::process::exit(13);
    }
    std::process::exit(0);
}