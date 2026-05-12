fn main() {
    let x = 7;
    let y = 3;

    // debug("Flag") expands to PP_NARG(__VA_ARGS__) which counts 1 argument
    if count_args!("Flag") != 1 {
        std::process::exit(1);
    }

    // debug("X = %d\n", x) counts 2 arguments
    if count_args!("X = %d\n", x) != -1 {
        std::process::exit(2);
    }

    // showlist(...) stringifies arguments
    let showlist_output = stringify_args!(The first, second, and third items.);
    if showlist_output != "The first, second, and third items." {
        std::process::exit(3);
    }

    // report(test, ...) returns 1 if test is true, else counts remaining arguments
    if report!(x > y, "x is {} but y is {}", x, y) != 1 {
        std::process::exit(4);
    }
    if report!(x < y, "x is {} but y is {}", x, y) != 3 {
        std::process::exit(5);
    }

    std::process::exit(0);
}

macro_rules! count_args {
    () => { 0 };
    ($first:tt $($rest:tt)*) => { 1 + count_args!($($rest)*) };
}

macro_rules! stringify_args {
    ($($args:tt)*) => { stringify!($($args)*) };
}

macro_rules! report {
    ($test:expr) => { if $test { 1 } else { 0 } };
    ($test:expr, $($args:tt)*) => { if $test { 1 } else { count_args!($($args)*) } };
}