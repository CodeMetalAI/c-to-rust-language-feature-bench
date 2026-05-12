use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

static I: AtomicI32 = AtomicI32::new(0);

fn main() {
    if I.load(Ordering::SeqCst) != 0 {
        exit(1);
    }
    I.store(7, Ordering::SeqCst);
    if I.load(Ordering::SeqCst) != 7 {
        exit(2);
    }
    exit(0);
}