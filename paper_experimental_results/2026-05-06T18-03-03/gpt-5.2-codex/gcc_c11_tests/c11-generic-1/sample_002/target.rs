use std::process;

fn check(n: i32) {
    if n != 0 {
        process::abort();
    }
}

fn f() {}

fn main() {
    let mut n: i32 = 0;

    check(0);
    check(n);

    check(0);
    check(n);

    let cn: i32 = 0;
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

    let _ = f as fn();
    check(0);
    check(n);

    let _ = process::abort as fn();
    check(0);
    check(n);

    let s: i16 = 0;
    let _ = s;
    check(0);
    check(n);

    process::exit(0);
}