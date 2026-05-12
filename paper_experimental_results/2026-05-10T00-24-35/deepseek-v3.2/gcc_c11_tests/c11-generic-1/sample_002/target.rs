fn abort() {
    std::process::exit(1);
}

fn check(n: i32) {
    if n != 0 {
        abort();
    }
}

fn main() {
    let mut n = 0;

    check(0); // _Generic(n++, int: 0) always yields 0 for int type
    check(n);

    check(0); // _Generic(n, double: n++, default: 0) yields default 0
    check(n);

    const cn: i32 = 0;
    check(0); // _Generic(cn, int: 0, default: n++) yields int case 0
    check(n);
    check(0); // _Generic((const int)n, int: 0, default: n++) yields int case 0
    check(n);

    let a: [i32; 1] = [0];
    const ca: [i32; 1] = [0];
    check(0); // _Generic(a, int *: 0, const int *: n++) yields int * case 0
    check(n);
    check(0); // _Generic(ca, const int *: 0, int *: n++) yields const int * case 0
    check(n);

    fn f() {}
    check(0); // _Generic(f, void (*)(void): 0, default: n++) yields void (*)(void) case 0
    check(n);

    check(0); // _Generic(&abort, void (*)(void): 0, default: n++) yields void (*)(void) case 0
    check(n);

    let s: i16 = 0;
    check(0); // _Generic(s, short: 0, int: n++) yields short case 0
    check(n);

    std::process::exit(0);
}