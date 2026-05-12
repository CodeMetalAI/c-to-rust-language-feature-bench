#![allow(dead_code)]

fn check<T>(n: T) {
    if n != 0 {
        std::process::exit(1);
    }
}

fn main() {
    let mut n = 0;

    check(_generic(n + 1));
    check(n);

    check(_generic(n, n + 1));
    check(n);

    let cn = 0;
    check(_generic(cn, 0, n + 1));
    check(n);
    check(_generic(&n, 0, n + 1));
    check(n);

    let a = [0];
    let ca = [0];
    check(_generic(&a, 0, n + 1));
    check(n);
    check(_generic(&ca, 0, n + 1));
    check(n);

    fn f() {}
    check(_generic(f, 0, n + 1));
    check(n);

    check(_generic(std::mem::forget(std::mem::ManuallyDrop::new(|| {})), 0, n + 1));
    check(n);

    let s: i16 = 0;
    check(_generic(s, 0, n + 1));
    check(n);

    std::process::exit(0);
}

fn _generic<T, U>(x: T, int: U, default: U) -> U
where
    T: Copy,
{
    match x {
        _ => default,
    }
}