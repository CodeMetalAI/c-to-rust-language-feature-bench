fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    // Equivalent to _Generic(n++, int: 0)
    check({
        let result = 0;
        n += 1;
        result
    });
    check(n);

    // Equivalent to _Generic(n, double: n++, default: 0)
    check({
        let result = 0;
        result
    });
    check(n);

    let cn: i32 = 0;
    // Equivalent to _Generic(cn, int: 0, default: n++)
    check(0);
    check(n);

    // Equivalent to _Generic((const int)n, int: 0, default: n++)
    check(0);
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    // Equivalent to _Generic(a, int *: 0, const int *: n++)
    check(0);
    check(n);

    // Equivalent to _Generic(ca, const int *: 0, int *: n++)
    check(0);
    check(n);

    fn f() {}
    // Equivalent to _Generic(f, void (*)(void): 0, default: n++)
    check(0);
    check(n);

    // Equivalent to _Generic(&abort, void (*)(void): 0, default: n++)
    check(0);
    check(n);

    let s: i16 = 0;
    // Equivalent to _Generic(s, short: 0, int: n++)
    check(0);
    check(n);

    std::process::exit(0);
}