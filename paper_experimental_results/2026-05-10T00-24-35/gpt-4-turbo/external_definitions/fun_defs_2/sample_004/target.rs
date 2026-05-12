fn main() {
    static mut COUNTER: i32 = 0;

    fn f() -> i32 {
        unsafe {
            COUNTER += 7;
            COUNTER
        }
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

    unsafe {
        COUNTER = 0;
    }

    let r1 = g_ptr(f);
    unsafe {
        if COUNTER != 14 {
            std::process::exit(1);
        }
    }
    if r1 != 21 {
        std::process::exit(2);
    }

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