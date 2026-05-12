use std::cell::RefCell;

static G: RefCell<i32> = RefCell::new(0);

fn set_g_return(gval: i32, ret: i32) -> i32 {
    *G.borrow_mut() = gval;
    ret
}

fn main() -> i32 {
    let mut y = 0;

    *G.borrow_mut() = 0;
    if set_g_return(0, 0) != 0 && { y += 1; true } {
        return 1;
    }
    if y != 0 {
        return 2;
    }

    *G.borrow_mut() = 0;
    if !(set_g_return(1, 1) != 0 && *G.borrow() == 1) {
        return 3;
    }

    0
}