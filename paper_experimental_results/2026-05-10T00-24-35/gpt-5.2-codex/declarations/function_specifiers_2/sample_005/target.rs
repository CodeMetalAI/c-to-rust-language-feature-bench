use std::sync::atomic::{AtomicI32, Ordering};

fn stop() -> ! {
    let x = AtomicI32::new(0);
    loop {
        x.fetch_add(1, Ordering::Relaxed);
    }
}

fn f() -> ! {
    stop();
}

fn g(i: i32) -> ! {
    if i > 0 {
        stop();
    }
    loop {}
}

fn main() {
    if 1 == 1 {
        f();
    }

    g(1);
}