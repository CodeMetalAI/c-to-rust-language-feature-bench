fn main() {
    let mut n = 0;

    check(n++);
    check(n);

    check(_generic(n, |d: double| n += 1, |_| 0));
    check(n);

    let cn = 0;
    check(_generic(cn, |i: i32| 0, |_| n += 1));
    check(n);
    check(_generic((cn as i32), |i: i32| 0, |_| n += 1));
    check(n);

    let a = [0; 1];
    let ca = [0; 1];
    check(_generic(&a, |p: *const i32| 0, |_| n += 1));
    check(n);
    check(_generic(&ca, |p: *const i32| 0, |_| n += 1));
    check(n);

    fn f() {}
    check(_generic(&f, |f: fn()| 0, |_| n += 1));
    check(n);

    check(_generic(&abort, |f: fn()| 0, |_| n += 1));
    check(n);

    let s: i16 = 0;
    check(_generic(s, |s: i16| 0, |_| n += 1));
    check(n);

    std::process::exit(0);
}

fn check(n: i32) {
    if n != 0 {
        abort();
    }
}

#[allow(dead_code)]
fn abort() {
    panic!("abort called");
}

fn _generic<T, U, R>(expr: T, match_cases: Vec<(std::any::TypeId, impl FnOnce(T) -> R)>, default_case: impl FnOnce(T) -> R) -> R {
    for (type_id, func) in match_cases {
        if expr.type_id() == type_id {
            return func(expr);
        }
    }
    default_case(expr)
}