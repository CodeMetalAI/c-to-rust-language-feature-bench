fn check(n: i32) {
    if n != 0 {
        panic!("abort");
    }
}

fn f() {}

fn main() {
    let mut n = 0;

    check({
        n += 1;
        0
    });
    check(n);

    check({
        let _ = 0.0; // To mimic the _Generic with double type
        0
    });
    check(n);

    let cn = 0;
    check({
        0
    });
    check(n);

    check({
        0
    });
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    check(0);
    check(n);
    check(0);
    check(n);

    check(0);
    check(n);

    check(0);
    check(n);

    let s: i16 = 0;
    check(0);
    check(n);
}