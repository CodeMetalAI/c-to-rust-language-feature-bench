use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

static G: AtomicI32 = AtomicI32::new(0);

fn set_g_return(gval: i32, ret: i32) -> i32 {
    G.store(gval, Ordering::SeqCst);
    ret
}

fn main() {
    let mut y: i32 = 0;

    G.store(0, Ordering::SeqCst);
    if set_g_return(0, 0) != 0 && { y += 1; y != 0 } {
        exit(1);
    }
    if y != 0 {
        exit(2);
    }

    G.store(0, Ordering::SeqCst);
    if !(set_g_return(1, 1) != 0 && (G.load(Ordering::SeqCst) == 1)) {
        exit(3);
    }

    exit(0);
}