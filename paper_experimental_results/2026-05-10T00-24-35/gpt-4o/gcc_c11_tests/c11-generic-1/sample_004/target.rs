fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    // Emulate _Generic behavior using match
    let result = match n {
        _ if n == n => 0, // Type match int
        _ => n,
    };
    check(result);
    check(n);

    // Second case
    let result = match n {
        _ if n == n => n,
        _ => {
            n += 1;
            0
        }
    };
    check(result);
    check(n);

    // Const test
    const CN: i32 = 0;
    let result = match CN {
        _ if CN == CN => 0, // Type match int
        _ => {
            n += 1;
            0
        }
    };
    check(result);
    check(n);

    let result = match n {
        _ if n == n => 0, // Type match int
        _ => {
            n += 1;
            0
        }
    };
    check(result);
    check(n);

    // Array tests
    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    let result = match a.as_ptr() {
        _ if a.as_ptr() as *const i32 == a.as_ptr() => 0, // Type match int *
        _ => {
            n += 1;
            0
        }
    };
    check(result);
    check(n);

    let result = match ca.as_ptr() {
        _ if ca.as_ptr() as *const i32 == ca.as_ptr() => 0, // Type match const int *
        _ => {
            n += 1;
            0
        }
    };
    check(result);
    check(n);

    // Function pointer tests
    extern fn f() {}
    let result = match f as fn() {
        _ if f as fn() == f as fn() => 0, // Type match void (*)(void)
        _ => {
            n += 1;
            0
        }
    };
    check(result);
    check(n);

    let result = match std::process::abort as fn() {
        _ if std::process::abort as fn() == std::process::abort as fn() => 0, // Type match void (*)(void)
        _ => {
            n += 1;
            0
        }
    };
    check(result);
    check(n);

    // Short type test
    let s: i16 = 0;
    let result = match s {
        _ if s == s => 0, // Type match short
        _ => {
            n += 1;
            0
        }
    };
    check(result);
    check(n);

    std::process::exit(0);
}