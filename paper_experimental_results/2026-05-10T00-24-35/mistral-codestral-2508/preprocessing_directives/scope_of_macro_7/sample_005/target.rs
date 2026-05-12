macro_rules! debug {
    ($($arg:expr),*) => {
        format_args!($($arg),*).count()
    };
}

macro_rules! showlist {
    ($($arg:expr),*) => {
        format!("{}", format_args!($($arg),*).as_str())
    };
}

macro_rules! report {
    ($test:expr, $($arg:expr),*) => {
        if $test {
            1
        } else {
            format_args!($($arg),*).count()
        }
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
    if debug!("X = {}\n", x) != 2 {
        std::process::exit(2);
    }

    if !streq(&showlist!("The first, second, and third items."), "The first, second, and third items.") {
        std::process::exit(3);
    }

    if report!(x > y, "x is {} but y is {}", x, y) != 1 {
        std::process::exit(4);
    }
    if report!(x < y, "x is {} but y is {}", x, y) != 3 {
        std::process::exit(5);
    }

    std::process::exit(0);
}