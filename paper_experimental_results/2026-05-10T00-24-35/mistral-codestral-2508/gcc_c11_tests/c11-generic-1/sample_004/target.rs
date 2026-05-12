fn main() {
    let mut n = 0;

    check(n);
    n += 1;
    check(n);

    check(n);
    check(n);

    let cn = 0;
    check(cn);
    check(n);
    check(n);
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    check(ca);
    check(n);
    check(a);
    check(n);

    extern "C" {
        fn f();
    }
    check(f);
    check(n);

    check(abort);
    check(n);

    let s: i16 = 0;
    check(s);
    check(n);

    std::process::exit(0);
}

fn check<T>(_arg: T) {}

fn abort() -> ! {
    panic!();
}