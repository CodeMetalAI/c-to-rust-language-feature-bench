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
    // 0 || true = true, so !(true) = false, condition is false, don't return 1
    let first_part = set_g_return(1, 0) != 0;
    let second_part = if first_part {
        true
    } else {
        G.with(|g| g.get() == 1)
    };
    if !(first_part || second_part) {
        std::process::exit(1);
    }

    G.with(|g| g.set(0));
    // set_g_return(0, 1) returns 1 (true), so short-circuit, don't evaluate ++y
    // Condition is true, so return 2
    let first_part = set_g_return(0, 1) != 0;
    let condition = if first_part {
        true
    } else {
        y += 1;
        y != 0
    };
    if condition {
        std::process::exit(2);
    }
    if y != 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}