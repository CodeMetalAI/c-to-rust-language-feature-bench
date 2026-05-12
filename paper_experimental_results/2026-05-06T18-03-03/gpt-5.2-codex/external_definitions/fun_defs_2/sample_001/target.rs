use std::sync::atomic::{AtomicI32, Ordering};
use std::process::exit;

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn f() -> i32 {
    COUNTER.fetch_add(7, Ordering::SeqCst) + 7
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
        exit(1);
    }
    if r1 != 21 {
        exit(2);
    }

    let r2 = g_fun(f);
    if COUNTER.load(Ordering::SeqCst) != 28 {
        exit(3);
    }
    if r2 != 49 {
        exit(4);
    }

    exit(0);
}