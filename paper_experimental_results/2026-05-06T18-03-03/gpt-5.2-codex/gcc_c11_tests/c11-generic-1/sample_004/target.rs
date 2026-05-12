fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n: i32 = 0;

    // _Generic(n++, int: 0)
    check(0);
    check(n);

    // _Generic(n, double: n++, default: 0)
    check(0);
    check(n);

    let _cn: i32 = 0;
    // _Generic(cn, int: 0, default: n++)
    check(0);
    check(n);
    // _Generic((const int)n, int: 0, default: n++)
    check(0);
    check(n);

    let _a: [i32; 1] = [0];
    let _ca: [i32; 1] = [0];
    // _Generic(a, int *: 0, const int *: n++)
    check(0);
    check(n);
    // _Generic(ca, const int *: 0, int *: n++)
    check(0);
    check(n);

    fn _f() {}
    // _Generic(f, void (*)(void): 0, default: n++)
    check(0);
    check(n);

    // _Generic(&abort, void (*)(void): 0, default: n++)
    check(0);
    check(n);

    let _s: i16 = 0;
    // _Generic(s, short: 0, int: n++)
    check(0);
    check(n);

    std::process::exit(0);
}