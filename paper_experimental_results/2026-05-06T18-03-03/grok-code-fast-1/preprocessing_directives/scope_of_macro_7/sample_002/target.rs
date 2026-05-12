macro_rules! count_args {
    () => { 0 };
    ($a:expr) => { 1 };
    ($a:expr, $($rest:expr),*) => { 1 + count_args!($($rest),*) };
}

macro_rules! debug {
    ($($args:expr),*) => { count_args!($($args),*) };
}

macro_rules! showlist {
    (The first, second, and third items.) => { "The first, second, and third items." };
}

macro_rules! report {
    ($test:expr, $($args:expr),*) => {
        if $test { 1 } else { count_args!($($args),*) }
    };
}

fn streq(a: &str, b: &str) -> bool {
    a == b
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

    if !streq(showlist!(The first, second, and third items.), "The first, second, and third items.") {
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