use std::process::exit;

fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    // Equivalent of _Generic(n++, int: 0)
    check(0);
    n += 1;
    check(n);

    // Equivalent of _Generic(n, double: n++, default: 0)
    check(0);
    check(n);

    let cn = 0;
    // _Generic(cn, int: 0, default: n++)
    check(0);
    check(n);
    // _Generic((const int)n, int: 0, default: n++)
    check(0);
    check(n);

    // _Generic(a, int *: 0, const int *: n++)
    check(0);
    check(n);
    // _Generic(ca, const int *: 0, int *: n++)
    check(0);
    check(n);

    // _Generic(f, void (*)(void): 0, default: n++)
    check(0);
    check(n);

    // _Generic(&abort, void (*)(void): 0, default: n++)
    check(0);
    check(n);

    let s = 0_i16;
    // _Generic(s, short: 0, int: n++)
    check(0);
    check(n);

    exit(0);
}