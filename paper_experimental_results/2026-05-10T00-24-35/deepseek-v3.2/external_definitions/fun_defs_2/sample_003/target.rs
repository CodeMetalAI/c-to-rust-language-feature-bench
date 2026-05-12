use std::cell::Cell;

static COUNTER: Cell<i32> = Cell::new(0);

fn f() -> i32 {
    COUNTER.set(COUNTER.get() + 7);
    COUNTER.get()
}

fn g_ptr(func: fn() -> i32) -> i32 {
    let a = func();
    let b = func();
    a + b
}

fn g_fun(func: fn() -> i32) -> i32 {
    let a = func();
    let b = func();
    a + b
}

fn main() {
    COUNTER.set(0);

    let r1 = g_ptr(f);
    if COUNTER.get() != 14 {
        return std::process::exit(1);
    }
    if r1 != 21 {
        return std::process::exit(2);
    }

    let r2 = g_fun(f);
    if COUNTER.get() != 28 {
        return std::process::exit(3);
    }
    if r2 != 49 {
        return std::process::exit(4);
    }

    std::process::exit(0);
}