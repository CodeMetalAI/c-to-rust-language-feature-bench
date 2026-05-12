fn main() {
    let mut n = 0;

    check(match n {
        0 => 0,
        _ => panic!("assertion failed"),
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
    check(match (cn as i32) {
        0 => 0,
        _ => n += 1,
    });
    check(n);

    let a = [0];
    let ca = [0];
    check(match a.as_ptr() {
        _ if a.len() == 1 => 0,
        _ => n += 1,
    });
    check(n);
    check(match ca.as_ptr() {
        _ if ca.len() == 1 => 0,
        _ => n += 1,
    });
    check(n);

    check(match () {
        () => 0,
        _ => n += 1,
    });
    check(n);

    check(match std::ptr::null::<fn()>() {
        _ => 0,
        _ => n += 1,
    });
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
    if n!= 0 {
        std::process::abort();
    }
}