use std::process;

fn check(n: i32) {
    if n!= 0 {
        process::exit(1);
    }
}

fn main() {
    let mut n = 0;

    check(n + 1);
    check(n);

    check(n + 1);
    check(n);

    const CN: i32 = 0;
    check(CN);
    check(n + 1);
    check(n + 1);
    check(n);

    let mut a = [0];
    const CA: [i32; 1] = [0];
    check(&mut a);
    check(n + 1);
    check(&CA);
    check(n + 1);

    fn f() {}
    check(f);
    check(n + 1);

    check(&process::abort);
    check(n + 1);

    let s: i32 = 0;
    check(s);
    check(n + 1);

    process::exit(0);
}