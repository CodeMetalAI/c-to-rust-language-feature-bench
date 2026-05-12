fn main() {
    static mut counter: i32 = 0;

    unsafe {
        counter = 0;

        let r1 = g_ptr(f);
        if counter != 14 {
            return 1;
        }
        if r1 != 21 {
            return 2;
        }

        let r2 = g_fun(f);
        if counter != 28 {
            return 3;
        }
        if r2 != 49 {
            return 4;
        }

        0
    }
}

static fn f() -> i32 {
    unsafe {
        counter += 7;
        counter
    }
}

static fn g_ptr(funcp: fn() -> i32) -> i32 {
    unsafe {
        let a = funcp();
        let b = funcp();
        a + b
    }
}

static fn g_fun(func: fn() -> i32) -> i32 {
    unsafe {
        let a = func();
        let b = func();
        a + b
    }
}