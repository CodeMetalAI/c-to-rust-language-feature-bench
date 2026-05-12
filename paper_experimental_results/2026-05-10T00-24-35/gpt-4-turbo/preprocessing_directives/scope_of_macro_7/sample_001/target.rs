fn main() {
    let x = 7;
    let y = 3;

    if debug!("Flag") != 1 {
        return std::process::exit(1);
    }
    if debug!("X = {}", x) != 2 {
        return std::process::exit(2);
    }

    let list_content = showlist!("The first", "second", "and third items.");
    if list_content != "The first, second, and third items." {
        return std::process::exit(3);
    }

    if report!(x > y) != 1 {
        return std::process::exit(4);
    }
    if report!(x < y, "x is {} but y is {}", x, y) != 3 {
        return std::process::exit(5);
    }
}

macro_rules! debug {
    ($($arg:tt)*) => {
        {
            let args = format_args!($($arg)*);
            1 + format!("{}", args).matches(',').count()
        }
    }
}

macro_rules! showlist {
    ($($arg:expr),* $(,)*) => {
        concat!($(stringify!($arg), ", "),*)
    }
}

macro_rules! report {
    ($test:expr $(, $arg:expr)*) => {
        if $test {
            1
        } else {
            1 + [$(stringify!($arg),)*].len()
        }
    }
}