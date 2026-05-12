use std::process;

fn check(n: i32) {
    if n != 0 {
        process::abort();
    }
}

fn f() {}

fn main() {
    let n: i32 = 0;

    check(0);
    check(n);

    check(0);
    check(n);

    let cn: i32 = 0;
    check(0);
    check(n);
    let _ = cn;
    check(0);
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    check(0);
    check(n);
    let _ = a;
    check(0);
    check(n);
    let _ = ca;

    check(0);
    check(n);

    check(0);
    check(n);

    let s: i16 = 0;
    check(0);
    check(n);
    let _ = s;

    let _ = f;

    process::exit(0);
}