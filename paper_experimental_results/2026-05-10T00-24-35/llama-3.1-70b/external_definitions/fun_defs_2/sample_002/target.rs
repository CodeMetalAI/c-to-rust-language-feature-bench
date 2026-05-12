fn f() -> i32 {
    static mut COUNTER: i32 = 0;

    unsafe {
        COUNTER += 7;
        COUNTER
    }
}

fn g_ptr<F>(funcp: F) -> i32
where
    F: Fn() -> i32,
{
    let a = funcp();
    let b = funcp();
    a + b
}

fn g_fun<F>(func: F) -> i32
where
    F: Fn() -> i32,
{
    let a = func();
    let b = func();
    a + b
}

fn main() {
    let f_ptr = || unsafe { f() };
    let r1 = g_ptr(f_ptr);
    assert_eq!(r1, 21, "r1 should be 21");

    let r2 = g_fun(f_ptr);
    assert_eq!(r2, 49, "r2 should be 49");
}