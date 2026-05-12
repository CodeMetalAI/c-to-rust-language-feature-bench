fn check<T>(n: T) {
    if n != 0 {
        std::process::exit(1);
    }
}

fn main() {
    let mut n = 0;

    check(_generic(n + 1));
    check(n);

    check(_generic(n));
    check(n);

    let cn = 0;
    check(_generic(cn));
    check(n);
    check(_generic(cn));
    check(n);

    let mut a = [0];
    let ca = [0];
    check(_generic(&a));
    check(n);
    check(_generic(&ca));
    check(n);

    fn f() {}
    check(_generic(f));
    check(n);

    check(_generic(&std::process::abort));
    check(n);

    let s: i16 = 0;
    check(_generic(s));
    check(n);

    std::process::exit(0);
}

fn _generic<T, U: Copy>(x: T) -> U {
    match x {
        _ => 0,
    }
}