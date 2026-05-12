use std::cell::Cell;
use std::process::exit;

thread_local! {
    static COUNTER: Cell<i32> = Cell::new(0);
}

fn f() -> i32 {
    COUNTER.with(|c| {
        let val = c.get() + 7;
        c.set(val);
        val
    })
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
    COUNTER.with(|c| c.set(0));

    let r1 = g_ptr(f);
    let counter_val = COUNTER.with(|c| c.get());
    if counter_val != 14 {
        exit(1);
    }
    if r1 != 21 {
        exit(2);
    }

    let r2 = g_fun(f);
    let counter_val = COUNTER.with(|c| c.get());
    if counter_val != 28 {
        exit(3);
    }
    if r2 != 49 {
        exit(4);
    }

    exit(0);
}