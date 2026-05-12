static mut COUNTER: i32 = 0;

fn f() -> i32 {
    COUNTER += 7;
    COUNTER
}

fn g_ptr<F>(func: F) -> i32
where
    F: Fn() -> i32,
{
    let a = func();
    let b = func();
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
    COUNTER = 0;

    let r1 = g_ptr(f);
    if COUNTER != 14 {
        std::process::exit(1);
    }
    if r1 != 21 {
        std::process::exit(2);
    }

    let r2 = g_fun(f);
    if COUNTER != 28 {
        std::process::exit(3);
    }
    if r2 != 49 {
        std::process::exit(4);
    }

    std::process::exit(0);
}