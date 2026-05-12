use std::cell::Cell;

thread_local! {
    static G: Cell<i32> = Cell::new(0);
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    G.with(|g| g.set(gval));
    ret
}

fn main() {
    let x: i32;

    G.with(|g| g.set(0));
    let cond1 = set_g_return(1, 1);
    let x1 = if cond1 != 0 {
        if G.with(|g| g.get()) == 1 { 1 } else { 0 }
    } else {
        set_g_return(2, 1);
        0
    };
    if x1 != 1 {
        std::process::exit(1);
    }
    if G.with(|g| g.get()) != 1 {
        std::process::exit(2);
    }

    G.with(|g| g.set(0));
    let cond2 = set_g_return(1, 0);
    let x2 = if cond2 != 0 {
        set_g_return(2, 1);
        0
    } else {
        if G.with(|g| g.get()) == 1 { 1 } else { 0 }
    };
    if x2 != 1 {
        std::process::exit(3);
    }
    if G.with(|g| g.get()) != 1 {
        std::process::exit(4);
    }

    std::process::exit(0);
}