use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let code = run();
    exit(code);
}

fn run() -> i32 {
    let x = AtomicI32::new(41);

    let old = x.fetch_add(1, Ordering::SeqCst);
    if old != 41 {
        return 1;
    }
    if x.load(Ordering::SeqCst) != 42 {
        return 2;
    }

    let old = x.fetch_add(1, Ordering::SeqCst);
    if old != 42 {
        return 3;
    }
    if x.load(Ordering::SeqCst) != 43 {
        return 4;
    }

    x.store(100, Ordering::SeqCst);
    let y = x.fetch_add(1, Ordering::SeqCst) + 0;
    if y != 100 {
        return 5;
    }
    if x.load(Ordering::SeqCst) != 101 {
        return 6;
    }

    0
}