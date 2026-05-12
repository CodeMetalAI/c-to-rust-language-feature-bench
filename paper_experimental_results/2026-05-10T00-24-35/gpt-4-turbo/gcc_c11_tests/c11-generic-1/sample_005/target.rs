fn main() {
    let mut n = 0;

    // Simulate the _Generic selection via a function/method overload
    check(match_type(n));
    n += 1; // simulate n++ from _Generic(n++, int: 0)
    check(n);

    check(match_type(n as f64));
    n += 1; // simulate n++ from _Generic(n, double: n++, default: 0)
    check(n);

    const CN: i32 = 0;
    check(match_type(CN));
    n += 1; // simulate n++ from _Generic(cn, int: 0, default: n++)
    check(n);
    check(match_type(n as const i32));
    n += 1; // simulate n++ from _Generic((const int)n, int: 0, default: n++)
    check(n);

    let a = [0; 1];
    check(match_type_array(&a));
    check(n);
    let ca = [0; 1];
    check(match_type_array_const(&ca));
    n += 1; // simulate n++ from _Generic(ca, const int *: 0, int *: n++)
    check(n);

    check(match_type_function(f));
    check(n);
    check(match_type_function(abort));
    n += 1; // simulate n++ from _Generic(&abort, void (*)(void): 0, default: n++)
    check(n);

    let s: i16 = 0;
    check(match_type(s));
    n += 1; // simulate n++ from _Generic(s, short: 0, int: n++)
    check(n);

    std::process::exit(0);
}

fn match_type<T>(_: T) -> i32 {
    0
}

fn match_type_array<T>(_: &[T; 1]) -> i32 {
    0
}

fn match_type_array_const<T>(_: &[T; 1]) -> i32 {
    0
}

fn match_type_function<T>(_: T) -> i32 {
    0
}

fn check(n: i32) {
    if n != 0 {
        std::process::abort();
    }
}

fn f() {}

fn abort() {
    std::process::abort();
}