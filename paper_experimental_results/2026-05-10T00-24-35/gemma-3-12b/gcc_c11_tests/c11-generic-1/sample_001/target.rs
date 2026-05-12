fn main() {
    let mut n = 0;

    check(n++);
    check(n);

    check(_generic(n, |d: f64| n += 1, |x| 0 as i32));
    check(n);

    let cn = 0;
    check(_generic(cn, |i: i32| 0, |x| n += 1));
    check(n);
    check(_generic((cn as i32), |i: i32| 0, |x| n += 1));
    check(n);

    let a = [0; 1];
    let ca = [0; 1];
    check(_generic(&a, |p: *const i32| 0, |p| n += 1));
    check(n);
    check(_generic(&ca, |p: *const i32| 0, |p| n += 1));
    check(n);

    fn f() {}
    check(_generic(&f, |f: fn()| 0, |x| n += 1));
    check(n);

    check(_generic(&abort, |f: fn()| 0, |x| n += 1));
    check(n);

    let s: i16 = 0;
    check(_generic(s, |s: i16| 0, |i: i32| n += 1));
    check(n);
}

fn check(n: i32) {
    if n != 0 {
        abort();
    }
}

#[allow(dead_code)]
fn abort() {
    panic!("abort called");
}

// Simple macro to emulate _Generic
macro_rules! _generic {
    ($val:expr, $( $type:tt $($match:tt)* ),?) => {
        {
            match $val {
                $(
                    if std::any::TypeId::of::<_>() == std::any::TypeId::of::<$type> {
                        $($match)*
                    }
                )?,
            }
            0
        }
    };
}