static mut counter: i32 = 0;

fn f() -> i32 {
    counter += 7;
    counter
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