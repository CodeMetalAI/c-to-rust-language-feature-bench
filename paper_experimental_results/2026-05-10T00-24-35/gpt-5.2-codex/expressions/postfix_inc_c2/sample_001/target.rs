use std::sync::atomic::{AtomicI32, Ordering};
use std::process::exit;

fn main() {
    let x = AtomicI32::new(41);

    let old = x.fetch_add(1, Ordering::SeqCst);
    if old != 41 {
        exit(1);
    }
    if x.load(Ordering::SeqCst) != 42 {
        exit(2);
    }

    let old = x.fetch_add(1, Ordering::SeqCst);
    if old != 42 {
        exit(3);
    }
    if x.load(Ordering::SeqCst) != 43 {
        exit(4);
    }

    x.store(100, Ordering::SeqCst);
    let y = x.fetch_add(1, Ordering::SeqCst) + 0;
    if y != 100 {
        exit(5);
    }
    if x.load(Ordering::SeqCst) != 101 {
        exit(6);
    }
}