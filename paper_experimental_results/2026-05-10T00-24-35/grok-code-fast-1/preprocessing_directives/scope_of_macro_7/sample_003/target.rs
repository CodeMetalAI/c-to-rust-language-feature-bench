macro_rules! count_args {
    () => {0};
    ($a:expr) => {1};
    ($a:expr, $b:expr) => {2};
    ($a:expr, $b:expr, $c:expr) => {3};
    ($a:expr, $b:expr, $c:expr, $d:expr) => {4};
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {5};
}

macro_rules! debug {
    ($($args:tt)*) => { count_args!($($args)*) }
}

macro_rules! showlist {
    ($($args:tt)*) => { stringify!($($args)*) }
}

macro_rules! report {
    ($test:expr) => { if $test {1} else {0} };
    ($test:expr, $($args:tt)*) => { if $test {1} else { count_args!($($args)*) } };
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