fn main() {
    let mut n = 0;

    check(n++);
    check(n);

    check(_generic(n, |d: double| n += 1, |_| 0));
    check(n);

    let cn = 0;
    check(_generic(cn, |i: &i32| 0, |_| n += 1));
    check(n);
    check(_generic((cn as i32), |i: &i32| 0, |_| n += 1));
    check(n);

    let a = [0; 1];
    let ca = []; // Empty array to satisfy const int[1]
    check(_generic(&a, |p: *const i32| 0, |_| n += 1));
    check(n);
    check(_generic(&ca, |p: *const i32| 0, |_| n += 1));
    check(n);

    // Placeholder for function f.  Rust doesn't have extern void f() easily.
    // We don't actually call it, so it's safe to just ignore it.
    check(_generic(&f, |func: fn()| 0, |_| n += 1));
    check(n);

    check(_generic(&abort, |func: fn()| 0, |_| n += 1));
    check(n);

    let s: i16 = 0;
    check(_generic(s, |i: i16| 0, |_| n += 1));
    check(n);

    std::process::exit(0);
}

fn check(n: i32) {
    if n != 0 {
        panic!("Assertion failed");
    }
}

// Simplified _Generic macro
macro_rules! _generic {
    ($expr:expr, $( $type:ty => $val:expr ),*) => {
        {
            $(
                if std::any::TypeId::of::<_>() == std::any::TypeId::of::<$type>() {
                    $val
                }
            )*
            0 // Default case
        }
    };
}

fn abort() {
    panic!("abort called");
}

fn f() {}