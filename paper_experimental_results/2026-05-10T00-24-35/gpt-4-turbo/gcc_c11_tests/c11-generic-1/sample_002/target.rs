fn main() {
    fn check(n: i32) {
        if n != 0 {
            panic!();
        }
    }

    let mut n = 0;

    // _Generic(n++, int: 0)
    // Since n is of type int, the result would be 0.
    // n++ in C is equivalent to a post-increment.
    let _ = n;
    n += 1;
    check(0);
    check(n);

    // _Generic(n, double: n++, default: 0)
    // Type of n is int, not double, so default case 0 is taken.
    check(0);
    check(n);

    // const int cn = 0;
    let cn = 0;
    // _Generic(cn, int: 0, default: n++)
    // cn is const int, matches int: 0.
    check(0);
    check(n);
    // _Generic((const int)n, int: 0, default: n++)
    // (const int)n is int, matches int: 0.
    check(0);
    check(n);

    // int a[1]; const int ca[1];
    let a = [0i32; 1];
    let ca = [0i32; 1];
    // _Generic(a, int *: 0, const int *: n++)
    check(0);
    check(n);
    // _Generic(ca, const int *: 0, int *: n++)
    check(0);
    check(n);

    // extern void f(void);
    fn f() {}
    // _Generic(f, void (*)(void): 0, default: n++)
    check(0);
    check(n);

    // _Generic(&abort, void (*)(void): 0, default: n++)
    check(0);
    check(n);

    // short s;
    let s: i16 = 0;
    // _Generic(s, short: 0, int: n++)
    check(0);
    check(n);

    std::process::exit(0);
}