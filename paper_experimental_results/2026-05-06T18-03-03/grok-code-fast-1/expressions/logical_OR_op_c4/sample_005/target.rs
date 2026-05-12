use std::cell::RefCell;
use std::process;

thread_local! {
    static G: RefCell<i32> = RefCell::new(0);
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    G.with(|g| *g.borrow_mut() = gval);
    ret
}

fn main() {
    let mut y = 0;

    G.with(|g| *g.borrow_mut() = 0);
    let condition1 = set_g_return(1, 0) != 0 || G.with(|g| *g.borrow() == 1);
    if !condition1 {
        process::exit(1);
    }

    G.with(|g| *g.borrow_mut() = 0);
    let condition2 = set_g_return(0, 1) != 0 || {
        y += 1;
        y != 0
    };
    if condition2 {
        process::exit(2);
    }
    if y != 0 {
        process::exit(3);
    }
}