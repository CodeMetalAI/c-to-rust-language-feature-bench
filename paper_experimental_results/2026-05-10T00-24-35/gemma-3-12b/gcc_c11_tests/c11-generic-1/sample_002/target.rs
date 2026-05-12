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

    extern "C" {
        fn f();
    }
    check(_generic(f, |func: unsafe extern "C" fn()| 0, |_| n += 1));
    check(n);

    check(_generic(&abort, |func: unsafe extern "C" fn()| 0, |_| n += 1));
    check(n);

    let s: i16 = 0;
    check(_generic(s, |sh: i16| 0, |i: i32| n += 1));
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

fn _generic<T>(val: T, f1: impl Fn(&T) -> i32, f2: impl Fn(&T) -> i32) -> i32 {
    match val {
        _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() => f1(&val),
        _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<double>() => f1(&val),
        _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<*const i32>() => f1(&val),
        _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<*const i32>() => f1(&val),
        _ if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i16>() => f1(&val),
        _ => f2(&val),
    }
}