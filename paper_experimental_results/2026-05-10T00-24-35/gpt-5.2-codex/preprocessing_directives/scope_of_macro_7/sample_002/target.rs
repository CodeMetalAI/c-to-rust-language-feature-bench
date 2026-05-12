use std::process::exit;

macro_rules! count_args {
    () => { 0i32 };
    ($head:expr $(, $tail:expr)*) => { 1i32 + count_args!($($tail),*) };
}

macro_rules! debug {
    ($($args:expr),* $(,)?) => { count_args!($($args),*) };
}

macro_rules! showlist {
    ($($t:tt)*) => {{
        let s = stringify!($($t)*);
        c_stringify(s)
    }};
}

macro_rules! report {
    ($test:expr $(, $args:expr)* $(,)?) => {{
        if $test { 1i32 } else { count_args!($($args),*) }
    }};
}

fn c_stringify(s: &str) -> String {
    let mut out = s.to_string();
    out = out.replace(" ,", ",");
    out = out.replace(" .", ".");
    out
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn main() {
    let x = 7;
    let y = 3;

    if debug!("Flag") != 1 {
        exit(1);
    }
    if debug!("X = %d\n", x) != 2 {
        exit(2);
    }

    let list = showlist!(The first, second, and third items.);
    if !streq(&list, "The first, second, and third items.") {
        exit(3);
    }

    if report!(x > y, "x is %d but y is %d", x, y) != 1 {
        exit(4);
    }
    if report!(x < y, "x is %d but y is %d", x, y) != 3 {
        exit(5);
    }

    exit(0);
}