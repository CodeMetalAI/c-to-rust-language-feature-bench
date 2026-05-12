fn main() {
    // Initialize the counter
    let mut counter = 0;

    // Closure to simulate the static function `f`
    let mut f = || {
        counter += 7;
        counter
    };

    // Function to simulate `g_ptr`
    let g_ptr = |func: &mut dyn FnMut() -> i32| {
        let a = func();
        let b = func();
        a + b
    };

    // Function to simulate `g_fun`
    let g_fun = |func: &mut dyn FnMut() -> i32| {
        let a = func();
        let b = func();
        a + b
    };

    // Testing behavior of `g_ptr(f)`
    let r1 = g_ptr(&mut f);
    if counter != 14 {
        std::process::exit(1);
    }
    if r1 != 21 {
        std::process::exit(2);
    }

    // Testing behavior of `g_fun(f)`
    let r2 = g_fun(&mut f);
    if counter != 28 {
        std::process::exit(3);
    }
    if r2 != 49 {
        std::process::exit(4);
    }

    // Successful execution
    std::process::exit(0);
}