use std::process::exit;

fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    check(0);
    check(n);

    check(0);
    check(n);

    const CN: i32 = 0;
    check(0);
    check(n);
    check(0);
    check(n);

    let a = [0; 1];
    let ca = [0; 1];
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

    exit(0);
}