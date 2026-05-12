fn check<T>(value: T) {
    if value != 0 {
        panic!("Unexpected non-zero value");
    }
}

fn exit() {
    std::process::exit(0);
}

fn main() {
    let mut n = 0;

    check(n);
    n += 1;
    check(n);

    let mut n = 0;
    check(n);
    n += 1;
    check(n);

    let constant_n = 0;
    check(constant_n);
    n += 1;
    check(n);
    check(constant_n);
    n += 1;
    check(n);

    let mut a = [1];
    let ca = [&a];
    check(&a);
    n += 1;
    check(n);
    check(&ca);
    n += 1;
    check(n);

    fn f() {}
    check(f);
    n += 1;
    check(n);

    check(abort);
    n += 1;
    check(n);

    let mut short_s = 0;
    check(short_s);
    n += 1;
    check(n);

    exit();
}

fn abort() {
    std::process::abort();
}