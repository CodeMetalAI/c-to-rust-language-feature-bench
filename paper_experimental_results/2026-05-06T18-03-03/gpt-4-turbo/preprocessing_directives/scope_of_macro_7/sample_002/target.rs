fn main() {
    let x = 7;
    let y = 3;

    if debug("Flag") != 1 {
        return std::process::exit(1);
    }
    if debug("X = %d\n", x) != 2 {
        return std::process::exit(2);
    }

    if showlist!("The first", "second", "and third items.") != "The first, second, and third items." {
        return std::process::exit(3);
    }

    if report(x > y, "x is {} but y is {}", x, y) != 1 {
        return std::process::exit(4);
    }
    if report(x < y, "x is {} but y is {}", x, y) != 3 {
        return std::process::exit(5);
    }
}

fn debug(args: std::fmt::Arguments) -> usize {
    std::fmt::format(args).split(',').count()
}

macro_rules! debug {
    ($($arg:tt)*) => (debug(format_args!($($arg)*)))
}

macro_rules! showlist {
    ($($item:expr),*) => {
        concat!($(stringify!($item), ", "),*)
    }
}

fn report<T: std::fmt::Display>(test: bool, args: std::fmt::Arguments) -> usize {
    if test {
        1
    } else {
        debug(args)
    }
}

macro_rules! report {
    ($test:expr, $($arg:tt)*) => (report($test, format_args!($($arg)*)))
}