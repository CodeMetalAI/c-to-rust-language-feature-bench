use lazy_static::lazy_static;
use std::cell::RefCell;

lazy_static! {
    static ref G: RefCell<i32> = RefCell::new(0);
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    *G.borrow_mut() = gval;
    ret
}

fn main() -> i32 {
    *G.borrow_mut() = 0;
    let x = if set_g_return(1, 1) != 0 {
        (*G.borrow() == 1) as i32
    } else {
        set_g_return(2, 1);
        0
    };
    if x != 1 {
        return 1;
    }
    if *G.borrow() != 1 {
        return 2;
    }

    *G.borrow_mut() = 0;
    let x = if set_g_return(1, 0) != 0 {
        set_g_return(2, 1);
        0
    } else {
        (*G.borrow() == 1) as i32
    };
    if x != 1 {
        return 3;
    }
    if *G.borrow() != 1 {
        return 4;
    }

    0
}