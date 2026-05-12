use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

static G: AtomicI32 = AtomicI32::new(0);

fn set_g_return(gval: i32, ret: i32) -> i32 {
    G.store(gval, Ordering::SeqCst);
    ret
}

fn main() {
    let mut x: i32;

    G.store(0, Ordering::SeqCst);
    x = if set_g_return(1, 1) != 0 {
        if G.load(Ordering::SeqCst) == 1 { 1 } else { 0 }
    } else {
        set_g_return(2, 1);
        0
    };
    if x != 1 {
        exit(1);
    }
    if G.load(Ordering::SeqCst) != 1 {
        exit(2);
    }

    G.store(0, Ordering::SeqCst);
    x = if set_g_return(1, 0) != 0 {
        set_g_return(2, 1);
        0
    } else {
        if G.load(Ordering::SeqCst) == 1 { 1 } else { 0 }
    };
    if x != 1 {
        exit(3);
    }
    if G.load(Ordering::SeqCst) != 1 {
        exit(4);
    }

    exit(0);
}