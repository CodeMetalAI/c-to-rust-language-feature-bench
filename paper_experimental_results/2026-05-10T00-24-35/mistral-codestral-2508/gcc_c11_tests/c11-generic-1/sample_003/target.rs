fn main() {
    let mut n = 0;

    // check(_Generic(n++, int: 0));
    n += 1;
    if n != 0 {
        std::process::exit(1);
    }
    // check(n);
    if n != 0 {
        std::process::exit(1);
    }

    // check(_Generic(n, double: n++, default: 0));
    if n != 0.0 {
        n += 1;
    } else {
        // default: 0
    }
    // check(n);
    if n != 0 {
        std::process::exit(1);
    }

    // const int cn = 0;
    let cn = 0;
    // check(_Generic(cn, int: 0, default: n++));
    if cn == 0 {
        // int: 0
    } else {
        n += 1;
    }
    // check(n);
    if n != 0 {
        std::process::exit(1);
    }
    // check(_Generic((const int)n, int: 0, default: n++));
    if n == 0 {
        // int: 0
    } else {
        n += 1;
    }
    // check(n);
    if n != 0 {
        std::process::exit(1);
    }

    // int a[1];
    let a: [i32; 1] = [0; 1];
    // const int ca[1];
    let ca: [i32; 1] = [0; 1];
    // check(_Generic(a, int *: 0, const int *: n++));
    if a.len() == 1 {
        // int *: 0
    } else {
        n += 1;
    }
    // check(n);
    if n != 0 {
        std::process::exit(1);
    }
    // check(_Generic(ca, const int *: 0, int *: n++));
    if ca.len() == 1 {
        // const int *: 0
    } else {
        n += 1;
    }
    // check(n);
    if n != 0 {
        std::process::exit(1);
    }

    // extern void f(void);
    extern "C" {
        fn f();
    }
    // check(_Generic(f, void (*)(void): 0, default: n++));
    if std::mem::size_of::<extern "C" fn()>() == std::mem::size_of::<fn()>() {
        // void (*)(void): 0
    } else {
        n += 1;
    }
    // check(n);
    if n != 0 {
        std::process::exit(1);
    }

    // check(_Generic(&abort, void (*)(void): 0, default: n++));
    if std::mem::size_of::<extern "C" fn()>() == std::mem::size_of::<fn()>() {
        // void (*)(void): 0
    } else {
        n += 1;
    }
    // check(n);
    if n != 0 {
        std::process::exit(1);
    }

    // short s;
    let s: i16 = 0;
    // check(_Generic(s, short: 0, int: n++));
    if std::mem::size_of::<i16>() == std::mem::size_of::<i32>() {
        // short: 0
    } else {
        n += 1;
    }
    // check(n);
    if n != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}