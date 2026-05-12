use std::cell::Cell;

thread_local! {
    static G: Cell<i32> = Cell::new(0);
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    G.with(|g| g.set(gval));
    ret
}

fn main() {
    let mut y = 0;

    G.with(|g| g.set(0));
    if set_g_return(0, 0) != 0 && { y += 1; y != 0 } {
        std::process::exit(1);
    }
    if y != 0 {
        std::process::exit(2);
    }

    G.with(|g| g.set(0));
    if !(set_g_return(1, 1) != 0 && G.with(|g| g.get()) == 1) {
        std::process::exit(3);
    }

    std::process::exit(0);
}