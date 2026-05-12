use std::sync::atomic::{AtomicI32, Ordering};
use std::process::exit;

static I0: AtomicI32 = AtomicI32::new(0);

fn main() {
    if I0.load(Ordering::SeqCst) != 0 {
        exit(1);
    }
    I0.store(7, Ordering::SeqCst);
    if I0.load(Ordering::SeqCst) != 7 {
        exit(2);
    }
    exit(0);
}