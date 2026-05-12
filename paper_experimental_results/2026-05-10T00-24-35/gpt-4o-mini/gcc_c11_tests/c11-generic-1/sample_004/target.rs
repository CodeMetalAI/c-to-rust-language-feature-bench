use std::process;

fn check(n: &mut i32, condition: bool) {
    if condition {
        process::abort();
    }
}

fn main() {
    let mut n = 0;

    check(&mut n, n != 0);
    check(&mut n, n != 0);

    check(&mut n, n as f64 != 0.0);
    check(&mut n, n != 0);

    const CN: i32 = 0;
    check(&mut n, CN != 0);
    check(&mut n, CN != 0);
    check(&mut n, CN != 0);
    check(&mut n, CN != 0);

    let a: [i32; 1] = [0];
    const CA: [i32; 1] = [0];
    check(&mut n, true);
    check(&mut n, true);
    check(&mut n, true); // both checks will increment n

    fn f() {}
    check(&mut n, true);
    check(&mut n, true);

    let mut s: i16 = 0;
    check(&mut n, s != 0);
    check(&mut n, n != 0);

    process::exit(0);
}