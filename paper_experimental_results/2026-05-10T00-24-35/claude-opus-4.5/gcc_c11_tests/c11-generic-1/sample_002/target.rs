fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n: i32 = 0;

    // _Generic(n++, int: 0) - n is int, so result is 0, n++ not evaluated in _Generic
    check(0);
    check(n);

    // _Generic(n, double: n++, default: 0) - n is int, matches default: 0
    check(0);
    check(n);

    let _cn: i32 = 0;
    // _Generic(cn, int: 0, default: n++) - const int is still int type, matches int: 0
    check(0);
    check(n);
    // _Generic((const int)n, int: 0, default: n++) - cast to const int is still int type
    check(0);
    check(n);

    let _a: [i32; 1] = [0; 1];
    let _ca: [i32; 1] = [0; 1];
    // _Generic(a, int *: 0, const int *: n++) - array decays to int*, matches int*: 0
    check(0);
    check(n);
    // _Generic(ca, const int *: 0, int *: n++) - const array decays to const int*, matches const int*: 0
    check(0);
    check(n);

    // _Generic(f, void (*)(void): 0, default: n++) - function decays to function pointer, matches
    check(0);
    check(n);

    // _Generic(&abort, void (*)(void): 0, default: n++) - &abort is void (*)(void), matches
    check(0);
    check(n);

    let _s: i16 = 0;
    // _Generic(s, short: 0, int: n++) - s is short, matches short: 0
    check(0);
    check(n);

    std::process::exit(0);
}