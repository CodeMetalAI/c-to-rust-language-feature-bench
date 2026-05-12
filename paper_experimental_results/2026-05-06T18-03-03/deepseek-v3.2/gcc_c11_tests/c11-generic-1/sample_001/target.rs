fn check(n: bool) {
    if n {
        std::process::exit(1);
    }
}

fn main() {
    let mut n = 0;

    check(std::mem::discriminant(&n) == std::mem::discriminant(&0_i32));
    check(n != 0);

    check(std::mem::discriminant(&n) == std::mem::discriminant(&0.0_f64));
    check(n != 0);

    let cn = 0;
    check(std::mem::discriminant(&cn) == std::mem::discriminant(&0_i32));
    check(n != 0);
    check(std::mem::discriminant(&(cn as i32)) == std::mem::discriminant(&0_i32));
    check(n != 0);

    let a = [0];
    let ca = [0];
    check(std::mem::discriminant(&a) == std::mem::discriminant(&[0_i32]));
    check(n != 0);
    check(std::mem::discriminant(&ca) == std::mem::discriminant(&[0_i32]));
    check(n != 0);

    fn f() {}
    check(std::mem::discriminant(&f) == std::mem::discriminant(&(|| {})));
    check(n != 0);

    check(std::mem::discriminant(&std::process::exit) == std::mem::discriminant(&(|| {})));
    check(n != 0);

    let s: i16 = 0;
    check(std::mem::discriminant(&s) == std::mem::discriminant(&0_i16));
    check(n != 0);

    std::process::exit(0);
}