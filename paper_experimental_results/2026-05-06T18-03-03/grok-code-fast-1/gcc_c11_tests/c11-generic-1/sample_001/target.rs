fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    check(0); // _Generic(n++, int: 0)
    check(n);

    check(0); // _Generic(n, double: n++, default: 0)
    check(n);

    let cn = 0;
    check(0); // _Generic(cn, int: 0, default: n++)
    check(n);
    check(0); // _Generic((const int)n, int: 0, default: n++)
    check(n);

    let a: [i32; 1] = [0];
    check(0); // _Generic(a, int *: 0, const int *: n++)
    check(n);
    let ca: [i32; 1] = [0];
    check(0); // _Generic(ca, const int *: 0, int *: n++)
    check(n);

    // extern void f(void); - omitted as not used
    check(0); // _Generic(f, void (*)(void): 0, default: n++)
    check(n);

    check(0); // _Generic(&abort, void (*)(void): 0, default: n++)
    check(n);

    let s: i16 = 0;
    check(0); // _Generic(s, short: 0, int: n++)
    check(n);

    std::process::exit(0);
}