use std::process;

fn check(n: i32) {
    if n != 0 {
        process::abort();
    }
}

fn main() {
    let mut n: i32 = 0;

    // _Generic(n++, int: 0) - In C, n++ is not evaluated in _Generic
    // The type is determined at compile time, so n is not incremented
    check(0); // int matches, returns 0
    check(n); // n is still 0

    // _Generic(n, double: n++, default: 0) - n is int, so default branch (0) is selected
    check(0);
    check(n); // n is still 0

    // const int cn = 0;
    // _Generic(cn, int: 0, default: n++) - cn is int (const qualifier ignored in _Generic)
    check(0);
    check(n); // n is still 0
    
    // _Generic((const int)n, int: 0, default: n++) - cast to const int, type is int
    check(0);
    check(n); // n is still 0

    // int a[1]; const int ca[1];
    // In _Generic, arrays decay to pointers
    // _Generic(a, int *: 0, const int *: n++) - a decays to int*, first branch matches
    check(0);
    check(n); // n is still 0
    
    // _Generic(ca, const int *: 0, int *: n++) - ca decays to const int*, first branch matches
    check(0);
    check(n); // n is still 0

    // _Generic(f, void (*)(void): 0, default: n++) - f is a function pointer type
    check(0);
    check(n); // n is still 0

    // _Generic(&abort, void (*)(void): 0, default: n++) - &abort is void (*)(void)
    check(0);
    check(n); // n is still 0

    // short s; _Generic(s, short: 0, int: n++) - s is short, first branch matches
    check(0);
    check(n); // n is still 0

    process::exit(0);
}