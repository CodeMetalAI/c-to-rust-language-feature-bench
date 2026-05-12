use std::sync::Mutex;

static COUNTER: Mutex<i32> = Mutex::new(0);

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
    let mut counter = COUNTER.lock().unwrap();
    *counter = 0;
    drop(counter);

    let r1 = g_ptr(f);
    let counter = COUNTER.lock().unwrap();
    if *counter != 14 {
        std::process::exit(1);
    }
    if r1 != 21 {
        std::process::exit(2);
    }
    drop(counter);

    let r2 = g_fun(f);
    let counter = COUNTER.lock().unwrap();
    if *counter != 28 {
        std::process::exit(3);
    }
    if r2 != 49 {
        std::process::exit(4);
    }
}