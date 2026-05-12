use std::sync::atomic::{AtomicI32, Ordering};
use std::process::exit;

static A: [i32; 100] = [0; 100];
static TABSIZE: AtomicI32 = AtomicI32::new(0);

fn main() {
    if A.len() != 100 {
        exit(1);
    }
    TABSIZE.store(7, Ordering::SeqCst);
    if TABSIZE.load(Ordering::SeqCst) != 7 {
        exit(2);
    }
    exit(0);
}