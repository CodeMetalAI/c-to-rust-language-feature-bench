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
        let temp = n;
        0
    });
    check(n);

    let cn = 0;
    check(0);
    check(n);
    check(0);
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    check(0);
    check(n);
    check(0);
    check(n);

    fn f() {}
    check(0);
    check(n);

    check(0);
    check(n);

    let s: i16 = 0;
    check(0);
    check(n);

    std::process::exit(0);
}