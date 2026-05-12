fn main() {
    let mut n = 0;

    check(n++);
    check(n);

    check(_generic(n, |d: double| n += 1, |_| 0));
    check(n);

    let cn = 0;
    check(_generic(cn, |i: i32| 0, |_| n += 1));
    check(n);
    check(_generic((cn as i32), |i: i32| 0, |_| n += 1));
    check(n);

    let a = [0; 1];
    let ca = [0; 1];
    check(_generic(&a, |p: *const i32| 0, |_| n += 1));
    check(n);
    check(_generic(&ca, |p: *const i32| 0, |_| n += 1));
    check(n);

    fn f() {}
    check(_generic(&f, |f: fn()| 0, |_| n += 1));
    check(n);

    check(_generic(&abort, |f: fn()| 0, |_| n += 1));
    check(n);

    let s: i16 = 0;
    check(_generic(s, |s: i16| 0, |_| n += 1));
    check(n);

    std::process::exit(0);
}

fn check(n: i32) {
    if n != 0 {
        abort();
    }
}

#[allow(dead_code)]
fn abort() {
    panic!("Aborted");
}

#[allow(unused_macros)]
macro_rules! _generic {
    ($expr:expr, $( $type:ty => $result:expr ),*) => {
        {
            $(
                if std::any::TypeId::of::<_>( &$expr ) == std::any::TypeId::of::< $type >() {
                    return $result;
                }
            )*
            0
        }
    };
}