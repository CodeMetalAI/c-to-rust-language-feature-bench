fn main() {
    let mut n = 0;

    check(type_switch(&n, 0));
    check(n);

    check(type_switch(&n as &dyn Any, n));
    n += 1;
    check(n);

    let cn = 0;
    check(type_switch(&cn, 0));
    check(n);
    check(type_switch(&(cn as i32), 0));
    check(n);

    let a = [0];
    let ca = [0];
    check(type_switch(&a as &[_], 0));
    check(n);
    check(type_switch(&ca as &[_], 0));
    check(n);

    check(type_switch(&f as &dyn Any, 0));
    check(n);

    check(type_switch(&abort as &dyn Any, 0));
    check(n);

    let s: i16 = 0;
    check(type_switch(&s, 0));
    check(n);

    std::process::exit(0);
}

fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn type_switch<T: ?Sized + Any>(value: &T, default: i32) -> i32 {
    match value.type_id() {
        _ if value.is::<i32>() => 0,
        _ if value.is::<f64>() => 1,
        _ if value.is::<[i32; 1]>() => 0,
        _ if value.is::<[i32; 1]>() => 1,
        _ if value.is::<fn()>() => 0,
        _ if value.is::<i16>() => 0,
        _ => default,
    }
}

fn f() {}

use std::any::Any;