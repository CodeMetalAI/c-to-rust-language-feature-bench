fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    // Equivalent to C's _Generic(n++, int: 0)
    check({
        let result = 0;
        n += 1;
        result
    });
    check(n);

    // Equivalent to C's _Generic(n, double: n++, default: 0)
    check({
        if false { // This arm is never selected, as n is not a double.
            n += 1;
            n - 1 // return value before increment
        } else {
            0
        }
    });
    check(n);

    let cn = 0;
    // Equivalent to C's _Generic(cn, int: 0, default: n++)
    check({
        if true { // cn is int
            0
        } else {
            n += 1;
            0
        }
    });
    check(n);

    // Equivalent to C's _Generic((const int)n, int: 0, default: n++)
    check({
        if true { // n is int
            0
        } else {
            n += 1;
            0
        }
    });
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];

    // Equivalent to C's _Generic(a, int *: 0, const int *: n++)
    check({
        if true { // a is int *
            0
        } else {
            n += 1;
            0
        }
    });
    check(n);

    // Equivalent to C's _Generic(ca, const int *: 0, int *: n++)
    check({
        if true { // ca is const int *
            0
        } else {
            n += 1;
            0
        }
    });
    check(n);

    extern "C" fn f() {}

    // Equivalent to C's _Generic(f, void (*)(void): 0, default: n++)
    check({
        if true { // f is a function pointer
            0
        } else {
            n += 1;
            0
        }
    });
    check(n);

    // Equivalent to C's _Generic(&abort, void (*)(void): 0, default: n++)
    check({
        if true { // &abort is a function pointer
            0
        } else {
            n += 1;
            0
        }
    });
    check(n);

    let s: i16 = 0;
    // Equivalent to C's _Generic(s, short: 0, int: n++)
    check({
        if true { // s is short
            0
        } else {
            n += 1;
            0
        }
    });
    check(n);

    std::process::exit(0);
}