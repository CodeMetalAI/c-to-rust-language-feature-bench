fn main() {
    let mut n = 0;

    check(match n {
        0 => 0,
        _ => unreachable!(),
    });
    n += 1;
    check(n);

    check(match n {
        _ => n += 1; 0,
    });
    check(n);

    let cn = 0;
    check(match cn {
        _ => 0,
    });
    check(n);
    check(match cn as i32 {
        _ => 0,
    });
    check(n);

    let mut a = [0; 1];
    let ca = [0; 1];
    check(match &a {
        _ => 0,
    });
    check(n);
    check(match &ca {
        _ => 0,
    });
    check(n);

    fn f() {}
    check(match f {
        _ => 0,
    });
    check(n);

    check(match std::mem::transmute::<_, fn()>(std::ptr::null_mut()) {
        _ => 0,
    });
    check(n);

    let s: i16 = 0;
    check(match s {
        _ => 0,
    });
    check(n);

    std::process::exit(0);
}

fn check(n: i32) {
    if n!= 0 {
        panic!();
    }
}