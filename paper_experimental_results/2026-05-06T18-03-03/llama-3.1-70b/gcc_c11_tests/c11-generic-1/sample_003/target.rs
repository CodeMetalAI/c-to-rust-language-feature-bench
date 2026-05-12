fn main() {
    let mut n = 0;

    check(match n {
        _ => 0,
    });
    n += 1;
    check(n);

    check(match n {
        0.0 => n += 1,
        _ => 0,
    });
    check(n);

    let cn = 0;
    check(match cn {
        0 => 0,
        _ => n += 1,
    });
    check(n);
    check(match cn as i32 {
        0 => 0,
        _ => n += 1,
    });
    check(n);

    let a = [0];
    let ca = [0];
    check(match a.as_ptr() {
        _ => 0,
    });
    n += 1;
    check(n);
    check(match ca.as_ptr() {
        _ => 0,
    });
    n += 1;
    check(n);

    extern "C" fn f() {}
    check(match f as fn() {
        _ => 0,
    });
    n += 1;
    check(n);

    check(match abort as fn() {
        _ => 0,
    });
    n += 1;
    check(n);

    let s: i16 = 0;
    check(match s {
        0 => 0,
        _ => n += 1,
    });
    check(n);

    std::process::exit(0);
}

fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn abort() {
    std::process::abort();
}