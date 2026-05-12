fn main() {
    let mut n = 0;

    // Equivalent to _Generic(n++, int: 0)
    n += 1;
    check(0, &mut n);

    check(n, &mut n);

    // Equivalent to _Generic(n, double: n++, default: 0)
    check(0, &mut n);

    check(n, &mut n);

    // Equivalent to _Generic(cn, int: 0, default: n++)
    check(0, &mut n);

    check(n, &mut n);

    // Equivalent to _Generic((const int)n, int: 0, default: n++)
    check(0, &mut n);

    check(n, &mut n);

    // Equivalent to _Generic(a, int *: 0, const int *: n++)
    check(0, &mut n);

    check(n, &mut n);

    // Equivalent to _Generic(ca, const int *: 0, int *: n++)
    check(0, &mut n);

    check(n, &mut n);

    // Equivalent to _Generic(f, void (*)(void): 0, default: n++)
    check(0, &mut n);

    check(n, &mut n);

    // Equivalent to _Generic(&abort, void (*)(void): 0, default: n++)
    check(0, &mut n);

    check(n, &mut n);

    // Equivalent to _Generic(s, short: 0, int: n++)
    check(0, &mut n);

    check(n, &mut n);

    std::process::exit(0);
}

fn check(n: i32, n_ref: &mut i32) {
    if n != 0 {
        std::process::abort();
    }
    *n_ref = n;
}