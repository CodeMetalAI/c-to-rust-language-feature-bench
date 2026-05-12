fn main() {
    let mut n = 0;

    check(n++);
    check(n);

    check(_generic(n, |d: double| n += 1, |default| default));
    check(n);

    let cn = 0;
    check(_generic(cn, |i: i32| 0, |default| n += 1));
    check(n);
    check(_generic((cn as i32), |i: i32| 0, |default| n += 1));
    check(n);

    let a = [0; 1];
    let ca = [0; 1];
    check(_generic(&a, |p: *const i32| 0, |default| n += 1));
    check(n);
    check(_generic(&ca, |p: *const i32| 0, |default| n += 1));
    check(n);

    // Simulate void f()
    check(_generic(&f, |f: fn()| 0, |default| n += 1));
    check(n);

    check(_generic(&abort, |f: fn()| 0, |default| n += 1));
    check(n);

    let s: i16 = 0;
    check(_generic(s, |_: i16| 0, |default| n += 1));
    check(n);

    std::process::exit(0);
}

fn check(n: i32) {
    if n != 0 {
        panic!("Assertion failed: n == 0");
    }
}

fn f() {
    // Dummy function for generic type checking
}

fn _generic<T, U>(expr: T, match_type: impl Fn(&T) -> U, default_case: impl FnOnce(T) -> U) -> U {
    let result = match expr {
        _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() => match_type(&expr),
        _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<double>() => match_type(&expr),
        _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<*const i32>() => match_type(&expr),
        _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<*const i32>() => match_type(&expr),
        _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<fn>() => match_type(&expr),
        _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i16>() => match_type(&expr),
        _ => default_case(expr),
    };
    result
}