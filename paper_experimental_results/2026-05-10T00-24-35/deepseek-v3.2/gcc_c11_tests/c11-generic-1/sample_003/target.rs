fn abort() -> ! {
    std::process::exit(1);
}

fn check(n: i32) {
    if n != 0 {
        abort();
    }
}

fn main() {
    let mut n = 0;

    check({
        let result = match n {
            _ => 0,
        };
        n += 1;
        result
    });
    check(n);

    check({
        match n {
            _ => 0,
        }
    });
    check(n);

    const CN: i32 = 0;
    check(match CN {
        _ => 0,
    });
    check(n);
    check(match {
        let val: i32 = n;
        val
    } {
        _ => 0,
    });
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    check(match &a as *const _ as *const i32 {
        _ => 0,
    });
    check(n);
    check(match &ca as *const _ as *const i32 {
        _ => 0,
    });
    check(n);

    fn f() {}
    check(match f as fn() {
        _ => 0,
    });
    check(n);

    check(match abort as fn() -> ! {
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