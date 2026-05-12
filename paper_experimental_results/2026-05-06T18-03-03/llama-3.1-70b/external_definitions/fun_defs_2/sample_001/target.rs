fn main() {
    let mut counter = 0;

    fn f() -> i32 {
        counter += 7;
        counter
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

    counter = 0;

    let r1 = g_ptr(f);
    if counter!= 14 {
        std::process::exit(1);
    }
    if r1!= 21 {
        std::process::exit(2);
    }

    let r2 = g_fun(f);
    if counter!= 28 {
        std::process::exit(3);
    }
    if r2!= 49 {
        std::process::exit(4);
    }

    std::process::exit(0);
}