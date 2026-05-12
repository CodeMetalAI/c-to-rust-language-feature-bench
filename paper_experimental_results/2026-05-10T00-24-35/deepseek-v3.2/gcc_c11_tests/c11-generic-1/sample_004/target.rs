fn main() {
    let mut n = 0;

    // _Generic(n++, int: 0)
    match n {
        _ => {
            let _ = n;
            n += 1;
            0
        }
    };

    check(n == 0);

    // _Generic(n, double: n++, default: 0)
    {
        let temp = n;
        match temp {
            _ => 0,
        }
    };
    check(n == 0);

    const CN: i32 = 0;
    // _Generic(cn, int: 0, default: n++)
    match CN {
        0 => 0,
        _ => {
            n += 1;
            n - 1
        }
    };
    check(n == 0);

    // _Generic((const int)n, int: 0, default: n++)
    {
        let temp = n as i32;
        match temp {
            _ => 0,
        }
    };
    check(n == 0);

    let a = [0i32];
    let ca = [0i32];
    // _Generic(a, int *: 0, const int *: n++)
    match &a as *const _ as *const i32 {
        _ => 0,
    };
    check(n == 0);

    // _Generic(ca, const int *: 0, int *: n++)
    match &ca as *const _ as *const i32 {
        _ => 0,
    };
    check(n == 0);

    // _Generic(f, void (*)(void): 0, default: n++)
    fn f() {}
    match f as fn() {
        _ => 0,
    };
    check(n == 0);

    // _Generic(&abort, void (*)(void): 0, default: n++)
    match abort as fn() -> ! {
        _ => 0,
    };
    check(n == 0);

    let s: i16 = 0;
    // _Generic(s, short: 0, int: n++)
    match s {
        _ => 0,
    };
    check(n == 0);

    std::process::exit(0);
}

fn abort() -> ! {
    std::process::abort();
}

fn check(cond: bool) {
    if !cond {
        abort();
    }
}