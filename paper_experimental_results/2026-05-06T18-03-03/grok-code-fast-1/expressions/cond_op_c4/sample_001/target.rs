#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;

lazy_static! {
    static ref G: RefCell<i32> = RefCell::new(0);
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    *G.borrow_mut() = gval;
    ret
}

fn main() {
    let mut x: i32;

    *G.borrow_mut() = 0;
    x = if set_g_return(1, 1) != 0 {
        (*G.borrow() == 1) as i32
    } else {
        set_g_return(2, 1);
        0
    };
    if x != 1 {
        std::process::exit(1);
    }
    if *G.borrow() != 1 {
        std::process::exit(2);
    }

    *G.borrow_mut() = 0;
    x = if set_g_return(1, 0) != 0 {
        set_g_return(2, 1);
        0
    } else {
        (*G.borrow() == 1) as i32
    };
    if x != 1 {
        std::process::exit(3);
    }
    if *G.borrow() != 1 {
        std::process::exit(4);
    }

    std::process::exit(0);
}