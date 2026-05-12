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

fn main() {
    unsafe {
        COUNTER = 0;
    }

    let r1 = g_ptr(f);
    if COUNTER!= 14 {
        panic!("Expected COUNTER to be 14, but got {}", COUNTER);
    }
    if r1!= 21 {
        panic!("Expected r1 to be 21, but got {}", r1);
    }

    let r2 = g_fun(f);
    if COUNTER!= 28 {
        panic!("Expected COUNTER to be 28, but got {}", COUNTER);
    }
    if r2!= 49 {
        panic!("Expected r2 to be 49, but got {}", r2);
    }

    println!("Test passed");
}