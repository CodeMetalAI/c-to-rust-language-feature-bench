use std::sync::atomic::{AtomicI32, Ordering};
use std::process;

static I: [AtomicI32; 1] = [AtomicI32::new(0)];

fn main() {
    let mut code = 0;
    if I[0].load(Ordering::SeqCst) != 0 {
        code = 1;
    } else {
        I[0].store(7, Ordering::SeqCst);
        if I[0].load(Ordering::SeqCst) != 7 {
            code = 2;
        }
    }
    process::exit(code);
}