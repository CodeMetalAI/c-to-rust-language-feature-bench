use std::sync::atomic::{AtomicI32, Ordering};

static G: AtomicI32 = AtomicI32::new(0);

fn die_if(x: i32) -> ! {
    let status = if x == 7 && G.load(Ordering::SeqCst) == 9 { 0 } else { 2 };
    std::process::exit(status);
}

fn main() {
    let p: fn(i32) -> ! = die_if;
    G.store(9, Ordering::SeqCst);
    p(7);
    std::process::exit(3);
}