fn main() {
    let mut n = 0;

    // Equivalent of _Generic(n++, int: 0) in C++
    n += 1;
    check(0);
    check(n);

    // Equivalent of _Generic(n, double: n++, default: 0)
    check(0);
    check(n);

    // Equivalent of _Generic(cn, int: 0, default: n++)
    const CN: i32 = 0;
    check(0);
    check(n);
    // Equivalent of _Generic((const int)n, int: 0, default: n++)
    check(0);
    check(n);

    // Equivalent of _Generic(a, int *: 0, const int *: n++)
    let mut a = [0; 1];
    check(0);
    check(n);
    // Equivalent of _Generic(ca, const int *: 0, int *: n++)
    let ca = [0; 1];
    check(0);
    check(n);

    // Equivalent of _Generic(f, void (*)(void): 0, default: n++)
    check(0);
    check(n);

    // Equivalent of _Generic(&abort, void (*)(void): 0, default: n++)
    check(0);
    check(n);

    // Equivalent of _Generic(s, short: 0, int: n++)
    let s: i16 = 0;
    check(0);
    check(n);

    // Normally exit(0);
    std::process::exit(0);
}

fn check(n: i32) {
    if n != 0 {
        abort();
    }
}

fn abort() -> ! {
    std::process::abort()
}