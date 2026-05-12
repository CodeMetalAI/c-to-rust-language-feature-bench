use std::cell::RefCell;
use std::process;

static COUNTER: RefCell<i32> = RefCell::new(0);

fn f() -> i32 {
    *COUNTER.borrow_mut() += 7;
    *COUNTER.borrow()
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
    *COUNTER.borrow_mut() = 0;

    let r1 = g_ptr(f);
    if *COUNTER.borrow() != 14 {
        process::exit(1);
    }
    if r1 != 21 {
        process::exit(2);
    }

    let r2 = g_fun(f);
    if *COUNTER.borrow() != 28 {
        process::exit(3);
    }
    if r2 != 49 {
        process::exit(4);
    }

    process::exit(0);
}