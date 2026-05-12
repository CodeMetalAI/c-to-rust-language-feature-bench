use std::any::TypeId;
use std::process;

fn check(n: i32) {
    if n != 0 {
        process::abort();
    }
}

fn main() {
    let mut n = 0i32;

    // check(_Generic(n++, int: 0));
    let temp = { let old = n; n += 1; old };
    let value = if TypeId::of::<i32>() == TypeId::of_val(&temp) {
        0
    } else {
        panic!()
    };
    check(value);
    check(n);

    // check(_Generic(n, double: n++, default: 0));
    let temp = n;
    let value = if TypeId::of::<f64>() == TypeId::of_val(&temp) {
        let old = n; n += 1; old
    } else {
        0
    };
    check(value);
    check(n);

    let cn = 0i32;
    // check(_Generic(cn, int: 0, default: n++));
    let temp = cn;
    let value = if TypeId::of::<i32>() == TypeId::of_val(&temp) {
        0
    } else {
        let old = n; n += 1; old
    };
    check(value);
    check(n);

    // check(_Generic((const int)n, int: 0, default: n++));
    let temp = n;
    let value = if TypeId::of::<i32>() == TypeId::of_val(&temp) {
        0
    } else {
        let old = n; n += 1; old
    };
    check(value);
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    // check(_Generic(a, int *: 0, const int *: n++));
    let temp = &a[0] as *mut i32;
    let value = if TypeId::of::<*mut i32>() == TypeId::of_val(&temp) {
        0
    } else if TypeId::of::<*const i32>() == TypeId::of_val(&temp) {
        let old = n; n += 1; old
    } else {
        panic!()
    };
    check(value);
    check(n);

    // check(_Generic(ca, const int *: 0, int *: n++));
    let temp = &ca[0] as *const i32;
    let value = if TypeId::of::<*const i32>() == TypeId::of_val(&temp) {
        0
    } else if TypeId::of::<*mut i32>() == TypeId::of_val(&temp) {
        let old = n; n += 1; old
    } else {
        panic!()
    };
    check(value);
    check(n);

    fn f() {}
    // check(_Generic(f, void (*)(void): 0, default: n++));
    let temp = f as fn();
    let value = if TypeId::of::<fn()>() == TypeId::of_val(&temp) {
        0
    } else {
        let old = n; n += 1; old
    };
    check(value);
    check(n);

    // check(_Generic(&abort, void (*)(void): 0, default: n++));
    let temp = process::abort as fn();
    let value = if TypeId::of::<fn()>() == TypeId::of_val(&temp) {
        0
    } else {
        let old = n; n += 1; old
    };
    check(value);
    check(n);

    let s: i16 = 0;
    // check(_Generic(s, short: 0, int: n++));
    let temp = s;
    let value = if TypeId::of::<i16>() == TypeId::of_val(&temp) {
        0
    } else if TypeId::of::<i32>() == TypeId::of_val(&temp) {
        let old = n; n += 1; old
    } else {
        panic!()
    };
    check(value);
    check(n);

    process::exit(0);
}