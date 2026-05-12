fn check<T>(n: T) {
    if n != 0 {
        std::process::exit(1);
    }
}

fn main() {
    let mut n = 0;

    check(_generic(n + 1, 0));
    check(n);

    check(_generic(n, n + 1));
    check(n);

    let cn = 0;
    check(_generic(cn, 0));
    check(n);
    check(_generic(&n, 0));
    check(n);

    let a = [0];
    let ca = [0];
    check(_generic(&a, 0));
    check(n);
    check(_generic(&ca, 0));
    check(n);

    fn f() {}
    check(_generic(f, 0));
    check(n);

    check(_generic(std::mem::forget(abort), 0));
    check(n);

    let s: i16 = 0;
    check(_generic(s, 0));
    check(n);

    std::process::exit(0);
}

fn _generic<T, U>(x: T, int: U) -> U {
    x
}

fn _generic<T, U>(x: T, double: U) -> U {
    x
}

fn _generic<T, U>(x: T, const_int: U) -> U {
    x
}

fn _generic<T, U>(x: T, default: U) -> U {
    x
}

fn _generic<T, U>(x: &T, int_ptr: U) -> U {
    x
}

fn _generic<T, U>(x: &T, const_int_ptr: U) -> U {
    x
}

fn _generic<T, U>(x: &T, default: U) -> U {
    x
}

fn _generic<T, U>(x: &dyn Fn() -> (), func: U) -> U {
    x
}

fn _generic<T, U>(x: &dyn Fn() -> (), default: U) -> U {
    x
}

fn _generic<T, U>(x: &dyn Fn() -> (), default: U) -> U {
    x
}

fn _generic<T, U>(x: &dyn Fn() -> (), default: U) -> U {
    x
}