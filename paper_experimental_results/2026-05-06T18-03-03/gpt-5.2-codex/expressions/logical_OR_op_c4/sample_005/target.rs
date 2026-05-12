use std::sync::atomic::{AtomicI32, Ordering};
use std::process;

static G: AtomicI32 = AtomicI32::new(0);

fn set_g_return(gval: i32, ret: i32) -> i32 {
    G.store(gval, Ordering::SeqCst);
    ret
}

fn main() {
    let mut y = 0;

    G.store(0, Ordering::SeqCst);
    if !(set_g_return(1, 0) != 0 || (G.load(Ordering::SeqCst) == 1)) {
        process::exit(1);
    }

    G.store(0, Ordering::SeqCst);
    if set_g_return(0, 1) != 0 || {
        y += 1;
        y != 0
    } {
        process::exit(2);
    }
    if y != 0 {
        process::exit(3);
    }

    process::exit(0);
}