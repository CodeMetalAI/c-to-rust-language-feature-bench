use std::process;

fn check(x: i32) {
    if x != 0 {
        process::abort();
    }
}

fn main() {
    let mut n = 0;

    check(0);
    check(n);

    check(0);
    check(n);

    let _cn = 0;
    check(0);
    check(n);
    check(0);
    check(n);

    let _a = [0i32; 1];
    let _ca = [0i32; 1];
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