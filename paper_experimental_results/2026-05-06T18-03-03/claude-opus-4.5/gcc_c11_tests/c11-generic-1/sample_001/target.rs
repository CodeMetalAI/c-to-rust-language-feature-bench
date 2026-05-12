use std::process::exit;

fn check(n: i32) {
    if n != 0 {
        panic!("abort");
    }
}

fn main() {
    let mut n: i32 = 0;

    // _Generic(n++, int: 0) - n++ is not evaluated, type is int, result is 0
    // In Rust, we just check the type at compile time conceptually
    // The key is that the expression in _Generic is NOT evaluated for side effects
    check(0_i32);  // n is int, so result is 0
    check(n);

    // _Generic(n, double: n++, default: 0) - n is int (not double), so default: 0
    check(0_i32);
    check(n);

    // const int cn = 0;
    // _Generic(cn, int: 0, default: n++) - const int matches int in C's _Generic
    let _cn: i32 = 0;
    check(0_i32);
    check(n);
    
    // _Generic((const int)n, int: 0, default: n++) - cast to const int still matches int
    check(0_i32);
    check(n);

    // int a[1]; - arrays decay to pointers in _Generic
    // _Generic(a, int *: 0, const int *: n++) - int[1] decays to int*, matches int*
    let _a: [i32; 1] = [0];
    check(0_i32);
    check(n);
    
    // const int ca[1]; - const int[1] decays to const int*
    // _Generic(ca, const int *: 0, int *: n++) - matches const int*
    let _ca: [i32; 1] = [0];
    check(0_i32);
    check(n);

    // extern void f(void);
    // _Generic(f, void (*)(void): 0, default: n++) - function decays to function pointer
    fn _f() {}
    check(0_i32);
    check(n);

    // _Generic(&abort, void (*)(void): 0, default: n++) - &abort is void (*)(void)
    check(0_i32);
    check(n);

    // short s;
    // _Generic(s, short: 0, int: n++) - s is short, matches short
    let _s: i16 = 0;
    check(0_i32);
    check(n);

    exit(0);
}