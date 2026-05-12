fn f(counter: &mut i32) -> i32 {
    *counter += 7;
    *counter
}

fn g_ptr(func: fn(&mut i32) -> i32, counter: &mut i32) -> i32 {
    let a = func(counter);
    let b = func(counter);
    a + b
}

fn g_fun(func: fn(&mut i32) -> i32, counter: &mut i32) -> i32 {
    let a = func(counter);
    let b = func(counter);
    a + b
}

fn main() {
    let mut counter = 0;

    let r1 = g_ptr(f, &mut counter);
    if counter != 14 {
        std::process::exit(1);
    }
    if r1 != 21 {
        std::process::exit(2);
    }

    let r2 = g_fun(f, &mut counter);
    if counter != 28 {
        std::process::exit(3);
    }
    if r2 != 49 {
        std::process::exit(4);
    }

    std::process::exit(0);
}