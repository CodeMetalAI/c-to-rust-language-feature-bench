use std::process;
use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let x = AtomicI32::new(41);

    let old = x.fetch_add(1, Ordering::SeqCst);
    if old != 41 {
        process::exit(1);
    }
    if x.load(Ordering::SeqCst) != 42 {
        process::exit(2);
    }

    let old = x.fetch_add(1, Ordering::SeqCst);
    if old != 42 {
        process::exit(3);
    }
    if x.load(Ordering::SeqCst) != 43 {
        process::exit(4);
    }

    x.store(100, Ordering::SeqCst);
    let y = x.fetch_add(1, Ordering::SeqCst) + 0;
    if y != 100 {
        process::exit(5);
    }
    if x.load(Ordering::SeqCst) != 101 {
        process::exit(6);
    }

    process::exit(0);
}