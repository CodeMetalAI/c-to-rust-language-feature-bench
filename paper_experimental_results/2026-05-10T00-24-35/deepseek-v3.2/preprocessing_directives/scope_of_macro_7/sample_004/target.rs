fn main() {
    let x = 7;
    let y = 3;

    // debug macro counts arguments
    if debug!("Flag") != 1 {
        std::process::exit(1);
    }
    if debug!("X = %d\n", x) != 2 {
        std::process::exit(2);
    }

    // showlist macro stringifies arguments
    if !streq(
        showlist!(The first, second, and third items.),
        "The first, second, and third items.",
    ) {
        std::process::exit(3);
    }

    // report macro returns 1 if test is true, otherwise counts arguments
    if report!(x > y, "x is {} but y is {}", x, y) != 1 {
        std::process::exit(4);
    }
    if report!(x < y, "x is {} but y is {}", x, y) != 3 {
        std::process::exit(5);
    }

    std::process::exit(0);
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}

macro_rules! debug {
    ($($arg:expr),*) => {
        count_args!($($arg),*)
    };
}

macro_rules! showlist {
    ($($arg:expr),*) => {
        concat!($(stringify!($arg),)*)
    };
}

macro_rules! report {
    ($test:expr, $($arg:expr),*) => {
        if $test { 1 } else { count_args!($($arg),*) }
    };
}

macro_rules! count_args {
    () => { 0 };
    ($_first:expr $(,$_rest:expr)*) => {
        1 + count_args!($($_rest),*)
    };
}