mod global {
    use std::cell::RefCell;
    pub static G: RefCell<i32> = RefCell::new(0);
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    *global::G.borrow_mut() = gval;
    ret
}

fn main() {
    let mut y = 0;

    *global::G.borrow_mut() = 0;
    if set_g_return(0, 0) != 0 && {
        y += 1;
        true
    } {
        std::process::exit(1);
    }
    if y != 0 {
        std::process::exit(2);
    }

    *global::G.borrow_mut() = 0;
    if !((set_g_return(1, 1) != 0) && (*global::G.borrow() == 1)) {
        std::process::exit(3);
    }

    std::process::exit(0);
}