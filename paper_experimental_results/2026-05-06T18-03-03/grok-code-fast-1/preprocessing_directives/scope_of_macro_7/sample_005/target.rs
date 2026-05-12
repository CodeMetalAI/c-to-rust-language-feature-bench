macro_rules! count {
    () => { 0 };
    ($x:expr $(, $xs:expr)*) => { 1 + count!($($xs),*) };
}

macro_rules! debug {
    ($($arg:expr),*) => { count!($($arg),*) };
}

macro_rules! showlist {
    ($($arg:expr),*) => { stringify!($($arg),*) };
}

macro_rules! report {
    ($test:expr, $($args:expr),*) => {
        if $test { 1 } else { count!($($args),*) }
    };
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn main() {
    let x: i32 = 7;
    let y: i32 = 3;

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
}