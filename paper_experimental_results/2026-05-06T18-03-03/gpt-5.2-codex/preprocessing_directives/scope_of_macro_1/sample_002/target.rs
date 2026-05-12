use std::sync::atomic::{AtomicI32, Ordering};

const TABSIZE_CONST: usize = 100;
static A: [i32; TABSIZE_CONST] = [0; TABSIZE_CONST];

static TABSIZE: AtomicI32 = AtomicI32::new(0);

fn main() {
    if A.len() != 100 {
        std::process::exit(1);
    }
    TABSIZE.store(7, Ordering::SeqCst);
    if TABSIZE.load(Ordering::SeqCst) != 7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}