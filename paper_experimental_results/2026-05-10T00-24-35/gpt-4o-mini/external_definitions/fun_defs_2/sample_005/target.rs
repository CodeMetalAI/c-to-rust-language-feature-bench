static mut COUNTER: i32 = 0;

fn f() -> i32 {
    unsafe {
        COUNTER += 7;
        COUNTER
    }
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

fn main() -> i32 {
    unsafe {
        COUNTER = 0;
    }

    let r1 = g_ptr(f);
    if unsafe { COUNTER } != 14 {
        return 1;
    }
    if r1 != 21 {
        return 2;
    }

    let r2 = g_fun(f);
    if unsafe { COUNTER } != 28 {
        return 3;
    }
    if r2 != 49 {
        return 4;
    }

    0
}