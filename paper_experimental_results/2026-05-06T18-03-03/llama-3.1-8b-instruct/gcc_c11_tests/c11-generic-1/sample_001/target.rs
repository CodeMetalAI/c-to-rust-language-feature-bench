fn check<T>(n: T) {
    if n != 0 {
        panic!("abort");
    }
}

fn main() {
    let mut n = 0;

    check(n + 1);
    check(n);

    check(if n == 0 { n + 1 } else { 0 });
    check(n);

    let cn = 0;
    check(if cn == 0 { 0 } else { n + 1 });
    check(n);
    check(if (cn as i32) == 0 { 0 } else { n + 1 });
    check(n);

    let a = [0];
    let ca = [0];
    check(if a.is_empty() { 0 } else { n + 1 });
    check(n);
    check(if ca.is_empty() { n + 1 } else { 0 });
    check(n);

    fn f() {}
    check(if f.is_some() { 0 } else { n + 1 });
    check(n);

    check(if std::mem::size_of::<fn()>() == 0 { 0 } else { n + 1 });
    check(n);

    let s: i16 = 0;
    check(if s == 0 { 0 } else { n + 1 });
    check(n);

    std::process::exit(0);
}