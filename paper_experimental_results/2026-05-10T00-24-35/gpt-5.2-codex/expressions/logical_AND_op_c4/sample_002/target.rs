use std::cell::Cell;
use std::process::exit;

thread_local! {
    static G: Cell<i32> = Cell::new(0);
}

fn set_g(gval: i32) {
    G.with(|g| g.set(gval));
}

fn get_g() -> i32 {
    G.with(|g| g.get())
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    set_g(gval);
    ret
}

fn main() {
    let mut y = 0;

    set_g(0);
    if set_g_return(0, 0) != 0 && {
        y += 1;
        y != 0
    } {
        exit(1);
    }
    if y != 0 {
        exit(2);
    }

    set_g(0);
    if !(set_g_return(1, 1) != 0 && (get_g() == 1)) {
        exit(3);
    }

    exit(0);
}