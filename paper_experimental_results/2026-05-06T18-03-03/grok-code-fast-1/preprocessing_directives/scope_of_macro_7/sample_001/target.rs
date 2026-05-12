macro_rules! count_args {
    () => { 0 };
    ($head:expr) => { 1 };
    ($head:expr, $($tail:expr),*) => { 1 + count_args!($($tail),*) };
}

macro_rules! debug {
    ($($arg:expr),*) => { count_args!($($arg),*) };
}

macro_rules! showlist {
    ($($arg:tt)*) => { stringify!($($arg)*) };
}

macro_rules! report {
    ($test:expr, $($args:expr),*) => {
        if $test { 1 } else { count_args!($($args),*) }
    };
}

fn streq(a: &str, b: &str) -> i32 {
    if a == b { 1 } else { 0 }
}

fn main() {
    let x = 7;
    let y = 3;

    if debug!("Flag") != 1 {
        std::process::exit(1);
    }
    if debug!("X = %d\n", x) != 2 {
        std::process::exit(2);
    }

    if streq(showlist!(The first, second, and third items.), "The first, second, and third items.") != 1 {
        std::process::exit(3);
    }

    if report!(x > y, "x is %d but y is %d", x, y) != 1 {
        std::process::exit(4);
    }
    if report!(x < y, "x is %d but y is %d", x, y) != 3 {
        std::process::exit(5);
    }

    std::process::exit(0);
}