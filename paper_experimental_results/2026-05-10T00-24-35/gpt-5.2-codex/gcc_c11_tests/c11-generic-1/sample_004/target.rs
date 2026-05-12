use std::process::{abort, exit};

fn check(n: i32) {
    if n != 0 {
        abort();
    }
}

fn f() {}

fn main() {
    let mut n: i32 = 0;

    check(0);
    check(n);

    check(0);
    check(n);

    const CN: i32 = 0;
    check(0);
    check(n);
    check(0);
    check(n);

    let _a: [i32; 1] = [0];
    let _ca: [i32; 1] = [0];
    check(0);
    check(n);
    check(0);
    check(n);

    let _fp: fn() = f;
    check(0);
    check(n);

    let _abort_ptr: fn() -> ! = abort;
    check(0);
    check(n);

    let _s: i16 = 0;
    check(0);
    check(n);

    exit(0);
}