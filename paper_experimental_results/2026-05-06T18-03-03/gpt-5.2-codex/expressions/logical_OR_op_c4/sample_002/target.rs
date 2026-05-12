use std::cell::Cell;

thread_local! {
    static G: Cell<i32> = Cell::new(0);
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    G.with(|g| g.set(gval));
    ret
}

fn get_g() -> i32 {
    G.with(|g| g.get())
}

fn main() {
    let mut y: i32 = 0;

    G.with(|g| g.set(0));
    if !(set_g_return(1, 0) != 0 || (get_g() == 1)) {
        std::process::exit(1);
    }

    G.with(|g| g.set(0));
    if set_g_return(0, 1) != 0 || { y += 1; y != 0 } {
        std::process::exit(2);
    }
    if y != 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}