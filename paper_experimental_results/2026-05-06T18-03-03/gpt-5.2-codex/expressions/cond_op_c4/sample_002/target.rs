use std::cell::Cell;
use std::process::exit;

thread_local! {
    static G: Cell<i32> = Cell::new(0);
}

fn set_g(val: i32) {
    G.with(|g| g.set(val));
}

fn get_g() -> i32 {
    G.with(|g| g.get())
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    set_g(gval);
    ret
}

fn main() {
    let mut x: i32;

    set_g(0);
    x = if set_g_return(1, 1) != 0 {
        (get_g() == 1) as i32
    } else {
        set_g_return(2, 1);
        0
    };
    if x != 1 {
        exit(1);
    }
    if get_g() != 1 {
        exit(2);
    }

    set_g(0);
    x = if set_g_return(1, 0) != 0 {
        set_g_return(2, 1);
        0
    } else {
        (get_g() == 1) as i32
    };
    if x != 1 {
        exit(3);
    }
    if get_g() != 1 {
        exit(4);
    }

    exit(0);
}