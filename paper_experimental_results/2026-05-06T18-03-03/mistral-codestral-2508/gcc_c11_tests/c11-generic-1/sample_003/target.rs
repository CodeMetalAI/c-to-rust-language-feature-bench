fn main() {
    let mut n = 0;

    // Equivalent to _Generic(n++, int: 0)
    n += 1;
    if n != 0 {
        panic!();
    }
    if n != 0 {
        panic!();
    }

    // Equivalent to _Generic(n, double: n++, default: 0)
    if n != 0 {
        panic!();
    }
    if n != 0 {
        panic!();
    }

    // Equivalent to _Generic(cn, int: 0, default: n++)
    let cn = 0;
    if cn != 0 {
        panic!();
    }
    if n != 0 {
        panic!();
    }

    // Equivalent to _Generic((const int)n, int: 0, default: n++)
    if n != 0 {
        panic!();
    }
    if n != 0 {
        panic!();
    }

    // Equivalent to _Generic(a, int *: 0, const int *: n++)
    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    if a.len() != 1 {
        panic!();
    }
    if n != 0 {
        panic!();
    }

    // Equivalent to _Generic(ca, const int *: 0, int *: n++)
    if ca.len() != 1 {
        panic!();
    }
    if n != 0 {
        panic!();
    }

    // Equivalent to _Generic(f, void (*)(void): 0, default: n++)
    fn f() {}
    if n != 0 {
        panic!();
    }
    if n != 0 {
        panic!();
    }

    // Equivalent to _Generic(&abort, void (*)(void): 0, default: n++)
    if n != 0 {
        panic!();
    }
    if n != 0 {
        panic!();
    }

    // Equivalent to _Generic(s, short: 0, int: n++)
    let s: i16 = 0;
    if s != 0 {
        panic!();
    }
    if n != 0 {
        panic!();
    }

    std::process::exit(0);
}