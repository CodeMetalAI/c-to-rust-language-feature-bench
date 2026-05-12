fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    // _Generic(n++, int: 0) equivalent in Rust
    check({
        n += 1;
        0
    });
    check(n);

    // _Generic(n, double: n++, default: 0) equivalent in Rust
    check({
        0
    });
    check(n);

    let cn = 0;
    // _Generic(cn, int: 0, default: n++) equivalent in Rust
    check({
        if cn is i32 { 0 } else { n += 1; n }
    });
    check(n);

    // _Generic((const int)n, int: 0, default: n++) equivalent in Rust
    check({
        if (n as i32) == n { 0 } else { n += 1; n }
    });
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    // _Generic(a, int *: 0, const int *: n++) equivalent in Rust
    check({
        if &a as *const _ as usize != 0 { 0 } else { n += 1; n }
    });
    check(n);

    // _Generic(ca, const int *: 0, int *: n++) equivalent in Rust
    check({
        if &ca as *const _ as usize != 0 { 0 } else { n += 1; n }
    });
    check(n);

    fn f() {}
    // _Generic(f, void (*)(void): 0, default: n++) equivalent in Rust
    check({
        if f as *const _ as usize != 0 { 0 } else { n += 1; n }
    });
    check(n);

    // _Generic(&abort, void (*)(void): 0, default: n++) equivalent in Rust
    check({
        if std::process::abort as *const _ as usize != 0 { 0 } else { n += 1; n }
    });
    check(n);

    let s: i16 = 0;
    // _Generic(s, short: 0, int: n++) equivalent in Rust
    check({
        if s is i16 { 0 } else { n += 1; n }
    });
    check(n);

    std::process::exit(0);
}