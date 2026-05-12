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
    // set_g_return(1, 0) returns 0 (false), so we evaluate g == 1
    // After set_g_return(1, 0), g is 1, so g == 1 is true
    // 0 || true = true, so !(true) = false, condition is false, we don't return 1
    if !(set_g_return(1, 0) != 0 || G.with(|g| g.get() == 1)) {
        std::process::exit(1);
    }

    G.with(|g| g.set(0));
    // set_g_return(0, 1) returns 1 (true), so short-circuit: ++y is NOT evaluated
    // The condition is true, so we return 2
    let cond = set_g_return(0, 1) != 0 || {
        y += 1;
        y != 0
    };
    if cond {
        std::process::exit(2);
    }
    if y != 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}