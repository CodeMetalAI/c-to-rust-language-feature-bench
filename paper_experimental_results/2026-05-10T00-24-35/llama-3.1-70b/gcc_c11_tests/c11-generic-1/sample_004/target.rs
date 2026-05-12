fn check(n: i32) {
    if n!= 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n: i32 = 0;

    check(match n {
        0..=i32::MAX => 0,
        _ => unreachable!(),
    });
    n += 1;
    check(n);

    match n {
        0.0f64.. => n += 1,
        _ => (),
    }
    check(n);

    let cn: i32 = 0;
    check(match cn {
        0..=i32::MAX => 0,
        _ => unreachable!(),
    });
    check(n);
    check(match cn as i32 {
        0..=i32::MAX => 0,
        _ => unreachable!(),
    });
    check(n);

    let mut a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    check(match a.as_mut_ptr() {
        _ if!a.as_mut_ptr().is_null() => 0,
        _ => unreachable!(),
    });
    check(n);
    check(match ca.as_ptr() {
        _ if!ca.as_ptr().is_null() => n += 1,
        _ => unreachable!(),
    });
    check(n);

    extern "C" {
        fn f();
    }
    check(match f as *const () {
        _ if!f.is_null() => 0,
        _ => unreachable!(),
    });
    check(n);

    check(match std::process::abort as *const () {
        _ if!std::process::abort.is_null() => 0,
        _ => unreachable!(),
    });
    check(n);

    let s: i16 = 0;
    check(match s {
        0..=i16::MAX => 0,
        _ => n += 1,
    });
    check(n);

    std::process::exit(0);
}