use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref COUNTER: Mutex<i32> = Mutex::new(0);
}

fn f() -> i32 {
    let mut c = COUNTER.lock().unwrap();
    *c += 7;
    *c
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
    *COUNTER.lock().unwrap() = 0;

    let r1 = g_ptr(f);
    if *COUNTER.lock().unwrap() != 14 {
        std::process::exit(1);
    }
    if r1 != 21 {
        std::process::exit(2);
    }

    let r2 = g_fun(f);
    if *COUNTER.lock().unwrap() != 28 {
        std::process::exit(3);
    }
    if r2 != 49 {
        std::process::exit(4);
    }

    std::process::exit(0);
}