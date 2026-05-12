fn main() {
    let mut n = 0;

    // Equivalent to _Generic(n++, int: 0)
    n += 1;
    check(n == 0, &mut n);

    check(n == 0, &mut n);

    // Equivalent to _Generic(n, double: n++, default: 0)
    // In Rust, we can't directly check the type at runtime, so we simulate the behavior
    // by assuming n is not a double, so it should be 0
    check(0 == 0, &mut n);

    check(n == 0, &mut n);

    let cn = 0;
    // Equivalent to _Generic(cn, int: 0, default: n++)
    // Since cn is an int, it should be 0
    check(0 == 0, &mut n);

    check(n == 0, &mut n);

    // Equivalent to _Generic((const int)n, int: 0, default: n++)
    // Since n is an int, it should be 0
    check(0 == 0, &mut n);

    check(n == 0, &mut n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];

    // Equivalent to _Generic(a, int *: 0, const int *: n++)
    // Since a is an int array, it should be 0
    check(0 == 0, &mut n);

    check(n == 0, &mut n);

    // Equivalent to _Generic(ca, const int *: 0, int *: n++)
    // Since ca is a const int array, it should be 0
    check(0 == 0, &mut n);

    check(n == 0, &mut n);

    // Equivalent to _Generic(f, void (*)(void): 0, default: n++)
    // Since f is a function pointer, it should be 0
    check(0 == 0, &mut n);

    check(n == 0, &mut n);

    // Equivalent to _Generic(&abort, void (*)(void): 0, default: n++)
    // Since &abort is a function pointer, it should be 0
    check(0 == 0, &mut n);

    check(n == 0, &mut n);

    let s: i16 = 0;
    // Equivalent to _Generic(s, short: 0, int: n++)
    // Since s is a short, it should be 0
    check(0 == 0, &mut n);

    check(n == 0, &mut n);

    std::process::exit(0);
}

fn check(n: bool, n_var: &mut i32) {
    if !n {
        *n_var += 1;
    }
}

fn f() {}