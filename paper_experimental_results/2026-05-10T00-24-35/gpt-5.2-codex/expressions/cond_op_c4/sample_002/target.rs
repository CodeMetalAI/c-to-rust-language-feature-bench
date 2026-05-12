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

fn bool_to_i32(b: bool) -> i32 {
    if b { 1 } else { 0 }
}

fn main() {
    let mut x: i32;

    G.with(|g| g.set(0));
    x = if set_g_return(1, 1) != 0 {
        bool_to_i32(get_g() == 1)
    } else {
        set_g_return(2, 1);
        0
    };
    if x != 1 {
        std::process::exit(1);
    }
    if get_g() != 1 {
        std::process::exit(2);
    }

    G.with(|g| g.set(0));
    x = if set_g_return(1, 0) != 0 {
        set_g_return(2, 1);
        0
    } else {
        bool_to_i32(get_g() == 1)
    };
    if x != 1 {
        std::process::exit(3);
    }
    if get_g() != 1 {
        std::process::exit(4);
    }

    std::process::exit(0);
}