fn check(n: i32) {
    if n!= 0 {
        panic!("abort");
    }
}

fn main() {
    let mut n: i32 = 0;

    check(0);
    n += 1;
    check(n);

    n += 1;
    check(n);
    n += 1;
    check(n);

    let cn: i32 = 0;
    check(0);
    check(n);
    check(0);
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    check(0);
    check(n);
    check(0);
    check(n);

    fn f() {}
    check(0);
    check(n);

    check(0);
    check(n);

    let s: i16 = 0;
    check(0);
    check(n);

    std::process::exit(0);
}