fn main() {
    let mut counter = 0;

    let f = || {
        counter += 7;
        counter
    };

    let g_ptr = |func: fn() -> int| {
        let a = func();
        let b = func();
        a + b
    };

    let g_fun = |func: fn() -> int| {
        let a = func();
        let b = func();
        a + b
    };

    counter = 0;

    let r1 = g_ptr(f);
    if counter != 14 {
        panic!("counter should be 14, but is {}", counter);
    }
    if r1 != 21 {
        panic!("r1 should be 21, but is {}", r1);
    }

    counter = 0;
    let r2 = g_fun(f);
    if counter != 28 {
        panic!("counter should be 28, but is {}", counter);
    }
    if r2 != 49 {
        panic!("r2 should be 49, but is {}", r2);
    }

    panic!("no error");
}