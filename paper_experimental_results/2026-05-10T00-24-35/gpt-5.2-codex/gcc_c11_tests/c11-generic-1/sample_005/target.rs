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

    let _cn: i32 = 0;
    check(0);
    check(n);
    check(0);
    check(n);

    let _a = [0; 1];
    let _ca = [0; 1];
    check(0);
    check(n);
    check(0);
    check(n);

    check(0);
    check(n);

    check(0);
    check(n);

    let _s: i16 = 0;
    check(0);
    check(n);

    process::exit(0);
}