fn main() {
    let x = 7;
    let y = 3;

    if debug!("Flag") != 1 {
        std::process::exit(1);
    }
    if debug!("X = {{}}", x) != 2 {
        std::process::exit(2);
    }

    if showlist!("The first, second, and third items.") != "The first, second, and third items." {
        std::process::exit(3);
    }

    if report!(x > y, "x is {{}} but y is {{}}", x, y) != 1 {
        std::process::exit(4);
    }
    if report!(x < y, "x is {{}} but y is {{}}", x, y) != 3 {
        std::process::exit(5);
    }
}

macro_rules! debug {
    ($($arg:tt)*) => {
        {
            let args = format_args!($($arg)*);
            eprintln!("{}", args);
            count_args!($($arg)*)
        }
    };
}

macro_rules! showlist {
    ($($arg:tt)*) => {
        format!("{}", stringify!($($arg)*))
    };
}

macro_rules! report {
    ($test:expr, $($arg:tt)*) => {
        if $test {
            1
        } else {
            let args = format_args!($($arg)*);
            eprintln!("{}", args);
            count_args!($($arg)*)
        }
    };
}

macro_rules! count_args {
    () => (0);
    ($_t:tt) => (1);
    ($_t:tt, $($rest:tt)*) => (1 + count_args!($($rest)*));
}