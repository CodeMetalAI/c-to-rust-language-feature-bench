fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    check(if let 0 = n { 0 } else { n += 1; 0 });
    check(n);

    check(if let 0.0 = n as f64 { n += 1; n } else { 0 });
    check(n);

    const CN: i32 = 0;
    check(if let 0 = CN { 0 } else { n += 1; 0 });
    check(n);
    check(if let 0 = n { 0 } else { n += 1; 0 });
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    check(if let 0 = a.as_ptr() as usize { 0 } else { n += 1; 0 });
    check(n);
    check(if let 0 = ca.as_ptr() as usize { 0 } else { n += 1; 0 });
    check(n);

    fn f() {}
    check(if let 0 = f as usize { 0 } else { n += 1; 0 });
    check(n);

    check(if let 0 = std::process::abort as usize { 0 } else { n += 1; 0 });
    check(n);

    let s: i16 = 0;
    check(if let 0 = s { 0 } else { n += 1; 0 });
    check(n);

    std::process::exit(0);
}