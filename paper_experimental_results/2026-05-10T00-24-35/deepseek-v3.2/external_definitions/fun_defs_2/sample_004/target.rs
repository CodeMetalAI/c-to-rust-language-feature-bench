static mut COUNTER: i32 = 0;

fn f() -> i32 {
    COUNTER += 7;
    COUNTER
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
    COUNTER = 0;

    let r1 = g_ptr(f);
    if COUNTER != 14 {
        return;
    }
    if r1 != 21 {
        return;
    }

    let r2 = g_fun(f);
    if COUNTER != 28 {
        return;
    }
    if r2 != 49 {
        return;
    }

    std::process::exit(0);
}