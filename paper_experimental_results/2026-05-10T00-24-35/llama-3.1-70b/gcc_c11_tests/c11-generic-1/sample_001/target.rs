fn check(n: i32) {
    if n!= 0 {
        panic!("Abort");
    }
}

fn main() {
    let mut n: i32 = 0;

    check(0);
    n += 1;
    check(n);

    check(match n {
        0.0f64 => n + 1,
        _ => 0,
    });
    check(n);

    const CN: i32 = 0;
    check(match CN {
        0 => 0,
        _ => n + 1,
    });
    check(n);
    check(match CN as i32 {
        0 => 0,
        _ => n + 1,
    });
    check(n);

    let a: [i32; 1] = [0];
    const CA: [i32; 1] = [0];
    check(match a.as_ptr() {
        _ if std::ptr::eq(a.as_ptr(), a.as_ptr()) => 0,
        _ => n + 1,
    });
    check(n);
    check(match CA.as_ptr() {
        _ if std::ptr::eq(CA.as_ptr(), CA.as_ptr()) => 0,
        _ => n + 1,
    });
    check(n);

    extern "C" fn f() {}
    check(match f as *const () {
        _ if std::ptr::eq(&f as *const _, &f as *const _) => 0,
        _ => n + 1,
    });
    check(n);

    check(match &|| {} as *const () {
        _ if std::ptr::eq(&|| {} as *const _, &|| {} as *const _) => 0,
        _ => n + 1,
    });
    check(n);

    let s: i16 = 0;
    check(match s {
        0 => 0,
        _ => n + 1,
    });
    check(n);

    std::process::exit(0);
}