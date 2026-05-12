fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    check({
        n += 1; // mimic n++
        0
    });
    check(n);

    check({
        0 // mimic _Generic(n, double: n++, default: 0)
    });
    check(n);

    let cn = 0;
    check({
        0 // mimic _Generic(cn, int: 0, default: n++)
    });
    check(n);
    check({
        0 // mimic _Generic((const int)n, int: 0, default: n++)
    });
    check(n);

    let a = [0; 1];
    let ca: [i32; 1] = [0; 1];
    check({
        0 // mimic _Generic(a, int *: 0, const int *: n++)
    });
    check(n);
    check({
        0 // mimic _Generic(ca, const int *: 0, int *: n++)
    });
    check(n);

    fn f() {}
    check({
        0 // mimic _Generic(f, void (*)(void): 0, default: n++)
    });
    check(n);

    check({
        0 // mimic _Generic(&abort, void (*)(void): 0, default: n++)
    });
    check(n);

    let s: i16 = 0;
    check({
        0 // mimic _Generic(s, short: 0, int: n++)
    });
    check(n);

    std::process::exit(0);
}