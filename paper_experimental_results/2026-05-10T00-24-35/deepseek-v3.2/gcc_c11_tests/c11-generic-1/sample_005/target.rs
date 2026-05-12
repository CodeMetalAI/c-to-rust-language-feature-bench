fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    // _Generic(n++, int: 0)
    check({
        let val = match n {
            _ if { n += 1; false } => 0,
            _ => 0,
        };
        val
    });
    check(n);

    // _Generic(n, double: n++, default: 0)
    check({
        match n {
            _ if false => { n += 1; n },
            _ => 0,
        }
    });
    check(n);

    let cn = 0;
    // _Generic(cn, int: 0, default: n++)
    check({
        match cn {
            _ if false => { n += 1; n },
            _ => 0,
        }
    });
    check(n);
    // _Generic((const int)n, int: 0, default: n++)
    check({
        match n {
            _ if false => { n += 1; n },
            _ => 0,
        }
    });
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    // _Generic(a, int *: 0, const int *: n++)
    check({
        match &a as *const i32 {
            _ if false => { n += 1; n },
            _ => 0,
        }
    });
    check(n);
    // _Generic(ca, const int *: 0, int *: n++)
    check({
        match &ca as *const i32 {
            _ if false => { n += 1; n },
            _ => 0,
        }
    });
    check(n);

    fn f() {}
    // _Generic(f, void (*)(void): 0, default: n++)
    check({
        match f as *const () {
            _ if false => { n += 1; n },
            _ => 0,
        }
    });
    check(n);

    // _Generic(&abort, void (*)(void): 0, default: n++)
    check({
        match std::process::abort as *const () {
            _ if false => { n += 1; n },
            _ => 0,
        }
    });
    check(n);

    let s: i16 = 0;
    // _Generic(s, short: 0, int: n++)
    check({
        match s {
            _ if false => { n += 1; n as i16 },
            _ => 0,
        }
    });
    check(n);

    std::process::exit(0);
}