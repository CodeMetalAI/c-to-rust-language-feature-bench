fn check<T>(n: T) -> ! {
    if n != T::default() {
        std::process::abort();
    }
}

fn main() {
    let mut n = 0;

    check(_generic(n, 0));
    check(n);

    check(_generic(n, 0));
    check(n);

    let cn = 0;
    check(_generic(cn, 0));
    check(n);
    check(_generic(cn, 0));
    check(n);

    let mut a = 0;
    let mut ca = 0;
    check(_generic(&mut a, 0));
    check(n);
    check(_generic(&mut ca, 0));
    check(n);

    let f = || {};
    check(_generic(f, 0));
    check(n);

    check(_generic(&mut std::process::abort as _, 0));
    check(n);

    let s = 0;
    check(_generic(s, 0));
    check(n);

    std::process::exit(0);
}

macro_rules! _generic {
    ($n:expr, $($key:expr => $val:expr),*) => {
        match $n {
            $( $key => $val, )*
            _ => panic!(),
        }
    };
}