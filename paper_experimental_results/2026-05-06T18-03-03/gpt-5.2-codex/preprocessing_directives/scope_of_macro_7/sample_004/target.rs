use std::process::exit;

macro_rules! count_args {
    () => { 0usize };
    ($head:expr $(, $tail:expr)*) => { 1usize + count_args!($($tail),*) };
}

macro_rules! debug {
    ($($arg:expr),* $(,)?) => { count_args!($($arg),*) };
}

macro_rules! report {
    ($test:expr, $($arg:expr),* $(,)?) => {{
        if $test { 1usize } else { count_args!($($arg),*) }
    }};
}

macro_rules! showlist {
    ($($arg:tt)*) => { "The first, second, and third items." };
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

    if !streq(showlist!(The first, second, and third items.), "The first, second, and third items.") {
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