use std::process;

fn check(n: i32) {
    if n != 0 {
        process::abort();
    }
}

fn main() {
    let mut n: i32 = 0;

    // _Generic(n++, int: 0) - n++ is not evaluated in _Generic
    check(0);
    check(n);

    // _Generic(n, double: n++, default: 0) - n is int, so default branch (0)
    check(0);
    check(n);

    // const int cn = 0;
    // _Generic(cn, int: 0, default: n++) - cn matches int, so 0
    check(0);
    check(n);
    
    // _Generic((const int)n, int: 0, default: n++) - matches int, so 0
    check(0);
    check(n);

    // int a[1];
    let a: [i32; 1] = [0];
    // const int ca[1];
    let ca: [i32; 1] = [0];
    
    // _Generic(a, int *: 0, const int *: n++) - array decays to int*, so 0
    check(0);
    check(n);
    
    // _Generic(ca, const int *: 0, int *: n++) - const array decays to const int*, so 0
    check(0);
    check(n);

    // extern void f(void);
    // _Generic(f, void (*)(void): 0, default: n++) - f is void(*)(void), so 0
    check(0);
    check(n);

    // _Generic(&abort, void (*)(void): 0, default: n++) - &abort is void(*)(void), so 0
    check(0);
    check(n);

    // short s;
    // _Generic(s, short: 0, int: n++) - s is short, so 0
    check(0);
    check(n);

    process::exit(0);
}