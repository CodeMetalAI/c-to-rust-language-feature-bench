use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<i32> = RefCell::new(0);
}

fn f() -> i32 {
    COUNTER.with(|c| {
        *c.borrow_mut() += 7;
        *c.borrow()
    })
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
    COUNTER.with(|c| *c.borrow_mut() = 0);

    let r1 = g_ptr(f);
    if COUNTER.with(|c| *c.borrow()) != 14 {
        std::process::exit(1);
    }
    if r1 != 21 {
        std::process::exit(2);
    }

    let r2 = g_fun(f);
    if COUNTER.with(|c| *c.borrow()) != 28 {
        std::process::exit(3);
    }
    if r2 != 49 {
        std::process::exit(4);
    }

    std::process::exit(0);
}