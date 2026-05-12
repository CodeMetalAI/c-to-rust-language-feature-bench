use std::cell::Cell;
use std::process;

thread_local! {
    static COUNTER: Cell<i32> = Cell::new(0);
}

fn f() -> i32 {
    COUNTER.with(|c| {
        let v = c.get() + 7;
        c.set(v);
        v
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

fn get_counter() -> i32 {
    COUNTER.with(|c| c.get())
}

fn set_counter(val: i32) {
    COUNTER.with(|c| c.set(val));
}

fn main() {
    set_counter(0);

    let r1 = g_ptr(f);
    if get_counter() != 14 {
        process::exit(1);
    }
    if r1 != 21 {
        process::exit(2);
    }

    let r2 = g_fun(f);
    if get_counter() != 28 {
        process::exit(3);
    }
    if r2 != 49 {
        process::exit(4);
    }

    process::exit(0);
}