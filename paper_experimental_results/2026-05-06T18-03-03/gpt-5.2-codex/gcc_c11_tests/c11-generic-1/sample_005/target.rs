fn check(n: i32) {
    if n != 0 {
        std::process::abort();
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

    std::process::exit(0);
}