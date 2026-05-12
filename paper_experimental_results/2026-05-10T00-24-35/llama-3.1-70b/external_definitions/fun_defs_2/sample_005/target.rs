static mut COUNTER: i32 = 0;

fn f() -> i32 {
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
    unsafe {
        COUNTER = 0;
    }

    let r1 = g_ptr(f);
    unsafe {
        assert_eq!(COUNTER, 14);
    }
    assert_eq!(r1, 21);

    let r2 = g_fun(f);
    unsafe {
        assert_eq!(COUNTER, 28);
    }
    assert_eq!(r2, 49);

    std::process::exit(0);
}