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

    let cn: i32 = 0;
    let _ = cn;
    check(0);
    check(n);
    check(0);
    check(n);

    let a = [0i32; 1];
    let ca = [0i32; 1];
    let _ = a;
    let _ = ca;
    check(0);
    check(n);
    check(0);
    check(n);

    let _fp: fn() = f;
    check(0);
    check(n);

    let _abort_ptr: fn() -> ! = std::process::abort;
    check(0);
    check(n);

    let s: i16 = 0;
    let _ = s;
    check(0);
    check(n);

    std::process::exit(0);
}