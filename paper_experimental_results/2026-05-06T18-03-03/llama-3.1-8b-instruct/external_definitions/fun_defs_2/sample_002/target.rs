fn main() {
    let mut counter = 0;

    let f = || {
        counter += 7;
        counter
    };

    let g_ptr = |func: fn() -> i32| {
        let a = func();
        let b = func();
        a + b
    };

    let g_fun = |func: fn() -> i32| {
        let a = func();
        let b = func();
        a + b
    };

    counter = 0;

    let r1 = g_ptr(f);
    if counter != 14 {
        panic!("counter != 14");
    }
    if r1 != 21 {
        panic!("r1 != 21");
    }

    counter = 0;
    let r2 = g_fun(f);
    if counter != 28 {
        panic!("counter != 28");
    }
    if r2 != 49 {
        panic!("r2 != 49");
    }

    assert!(true);
}