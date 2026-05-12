use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

static G: AtomicI32 = AtomicI32::new(0);

fn die_if(x: i32) -> ! {
    let g = G.load(Ordering::SeqCst);
    exit(if x == 7 && g == 9 { 0 } else { 2 });
}

fn main() {
    let p: fn(i32) -> ! = die_if;
    G.store(9, Ordering::SeqCst);
    p(7);
}