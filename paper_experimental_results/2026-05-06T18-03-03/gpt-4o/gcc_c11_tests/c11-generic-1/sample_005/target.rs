fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    check({
        n += 1;
        0
    });
    check(n);

    check({
        let _ = n;
        0
    });
    check(n);

    let cn = 0;
    check({
        let _ = cn;
        0
    });
    check(n);
    check({
        let _ = n as i32;
        0
    });
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    check({
        let _ = &a as *const i32;
        0
    });
    check(n);
    check({
        let _ = &ca as *const i32;
        0
    });
    check(n);

    fn f() {}
    check({
        let _ = f as fn();
        0
    });
    check(n);

    check({
        let _ = std::process::abort as fn();
        0
    });
    check(n);

    let s: i16 = 0;
    check({
        let _ = s;
        0
    });
    check(n);

    std::process::exit(0);
}