// fun_defs_2.rs

static mut COUNTER: i32 = 0;

fn f() -> i32 {
    unsafe {
        COUNTER += 7;
    }
    COUNTER
}

fn g_ptr<F>(func: F) -> i32
where
    F: Fn() -> i32 + Send + Sync,
{
    let a = func();
    let b = func();
    a + b
}

fn g_fun<F>(func: F) -> i32
where
    F: Fn() -> i32 + Send + Sync + 'static,
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
    if COUNTER != 14 {
        println!("Error 1");
        std::process::exit(1);
    }
    if r1 != 21 {
        println!("Error 2");
        std::process::exit(2);
    }

    let r2 = g_fun(f);
    if COUNTER != 28 {
        println!("Error 3");
        std::process::exit(3);
    }
    if r2 != 49 {
        println!("Error 4");
        std::process::exit(4);
    }

    println!("Success");
    std::process::exit(0);
}