use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<i32> = RefCell::new(0);
}

fn f() -> i32 {
    COUNTER.with(|c| {
        *c.borrow_mut() += 7;
        c.borrow().clone()
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
    COUNTER.with(|c| *c.borrow_mut() = 0);

    let r1 = g_ptr(f);
    let counter_val = COUNTER.with(|c| c.borrow().clone());
    if counter_val != 14 {
        std::process::exit(1);
    }
    if r1 != 21 {
        std::process::exit(2);
    }

    let r2 = g_fun(f);
    let counter_val = COUNTER.with(|c| c.borrow().clone());
    if counter_val != 28 {
        std::process::exit(3);
    }
    if r2 != 49 {
        std::process::exit(4);
    }

    std::process::exit(0);
}