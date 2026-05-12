macro_rules! count_args {
    () => { 0i32 };
    ($head:expr $(, $tail:expr)*) => { 1i32 + count_args!($($tail),*) };
}

macro_rules! debug {
    ($($args:expr),* $(,)?) => {
        count_args!($($args),*)
    };
}

macro_rules! report {
    ($test:expr $(, $rest:expr)*) => {
        if $test { 1i32 } else { count_args!($($rest),*) }
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

    let list = "The first, second, and third items.";
    if !streq(list, "The first, second, and third items.") {
        std::process::exit(3);
    }

    if report!(x > y, "x is %d but y is %d", x, y) != 1 {
        std::process::exit(4);
    }
    if report!(x < y, "x is %d but y is %d", x, y) != 3 {
        std::process::exit(5);
    }
}