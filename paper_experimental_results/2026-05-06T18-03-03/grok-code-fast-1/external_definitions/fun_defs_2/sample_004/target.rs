use std::sync::atomic::{AtomicI32, Ordering};

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn f() -> i32 {
    let old = COUNTER.fetch_add(7, Ordering::SeqCst);
    old + 7
}

fn g_ptr(funcp: fn() -> i32) -> i32 {
    let a = funcp();
    let b = funcp();
    a + b
}

fn g_fun(func: fn() -> i32) -> i32 {
    let a = func();
    let b = func();
    a + b
}

fn main() {
    COUNTER.store(0, Ordering::SeqCst);
    let r1 = g_ptr(f);
    if COUNTER.load(Ordering::SeqCst) != 14 {
        std::process::exit(1);
    }
    if r1 != 21 {
        std::process::exit(2);
    }
    let r2 = g_fun(f);
    if COUNTER.load(Ordering::SeqCst) != 28 {
        std::process::exit(3);
    }
    if r2 != 49 {
        std::process::exit(4);
    }
    std::process::exit(0);
}