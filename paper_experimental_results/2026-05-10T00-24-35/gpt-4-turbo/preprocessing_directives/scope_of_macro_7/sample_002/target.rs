fn main() {
    let x = 7;
    let y = 3;

    if debug!("Flag") != 1 {
        std::process::exit(1);
    }
    if debug!("X = {}", x) != 2 {
        std::process::exit(2);
    }

    if !streq(showlist!("The first", "second", "and third items."),
              "The first, second, and third items.") {
        std::process::exit(3);
    }

    if report!(x > y, "x is {} but y is {}", x, y) != 1 {
        std::process::exit(4);
    }
    if report!(x < y, "x is {} but y is {}", x, y) != 3 {
        std::process::exit(5);
    }
}

macro_rules! debug {
    ($($arg:tt)*) => ({
        let args = format_args!($($arg)*);
        eprintln!("{}", args);
        std::env::args().count() // Placeholder to mimic returning the number of arguments
    })
}

macro_rules! showlist {
    ($($arg:tt),*) => ({
        concat!($(stringify!($arg), ", "),*)
    })
}

macro_rules! report {
    ($test:expr, $($arg:tt)*) => ({
        if $test { 1 } else { debug!($($arg)*) }
    })
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}