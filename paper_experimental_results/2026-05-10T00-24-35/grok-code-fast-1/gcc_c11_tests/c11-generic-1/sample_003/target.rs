fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

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

    let _a: [i32; 1] = [0];
    let _ca: [i32; 1] = [0];
    check(0);
    check(n);
    check(0);
    check(n);

    extern "C" {
        fn f();
    }
    check(0);
    check(n);

    check(0);
    check(n);

    let _s: i16 = 0;
    check(0);
    check(n);

    std::process::exit(0);
}