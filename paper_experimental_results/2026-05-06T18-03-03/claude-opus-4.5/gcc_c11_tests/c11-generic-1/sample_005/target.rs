use std::process::exit;

fn check(n: i32) {
    if n != 0 {
        panic!("abort");
    }
}

fn main() {
    let mut n: i32 = 0;

    // _Generic(n++, int: 0) - n is int, so result is 0, and n++ is not evaluated
    // In Rust, we just check the type at compile time conceptually
    // Since n is i32, we return 0
    check(0_i32);
    check(n);

    // _Generic(n, double: n++, default: 0) - n is int (not double), so default: 0
    check(0_i32);
    check(n);

    // const int cn = 0;
    // _Generic(cn, int: 0, default: n++) - const int matches int, so 0
    let _cn: i32 = 0;
    check(0_i32);
    check(n);
    
    // _Generic((const int)n, int: 0, default: n++) - const int matches int, so 0
    check(0_i32);
    check(n);

    // int a[1]; - array decays to int*, matches int*: 0
    let _a: [i32; 1] = [0];
    // const int ca[1]; - array decays to const int*, matches const int*: 0
    let _ca: [i32; 1] = [0];
    check(0_i32);
    check(n);
    check(0_i32);
    check(n);

    // extern void f(void); - function decays to void (*)(void), matches: 0
    check(0_i32);
    check(n);

    // &abort is void (*)(void), matches: 0
    check(0_i32);
    check(n);

    // short s; _Generic(s, short: 0, int: n++) - s is short, matches short: 0
    let _s: i16 = 0;
    check(0_i32);
    check(n);

    exit(0);
}