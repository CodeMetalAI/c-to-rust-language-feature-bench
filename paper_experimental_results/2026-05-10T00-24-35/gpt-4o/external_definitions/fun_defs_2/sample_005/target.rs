fn main() {
    // Define a static mutable counter
    static mut COUNTER: i32 = 0;

    // Define function `f` that modifies the counter
    fn f() -> i32 {
        unsafe {
            COUNTER += 7;
            COUNTER
        }
    }

    // Define function `g_ptr` that takes a function pointer
    fn g_ptr(funcp: fn() -> i32) -> i32 {
        let a = funcp();
        let b = funcp();
        a + b
    }

    // Define function `g_fun` that takes a function pointer
    fn g_fun(func: fn() -> i32) -> i32 {
        let a = func();
        let b = func();
        a + b
    }

    // Reset the counter to 0
    unsafe {
        COUNTER = 0;
    }

    // Call g_ptr with f and check conditions
    let r1 = g_ptr(f);
    unsafe {
        if COUNTER != 14 {
            std::process::exit(1);
        }
    }
    if r1 != 21 {
        std::process::exit(2);
    }

    // Call g_fun with f and check conditions
    let r2 = g_fun(f);
    unsafe {
        if COUNTER != 28 {
            std::process::exit(3);
        }
    }
    if r2 != 49 {
        std::process::exit(4);
    }
}