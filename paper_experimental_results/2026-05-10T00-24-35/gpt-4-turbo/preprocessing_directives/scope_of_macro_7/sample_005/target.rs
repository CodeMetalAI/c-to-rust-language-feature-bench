fn main() {
    let x = 7;
    let y = 3;

    if debug("Flag") != 1 {
        std::process::exit(1);
    }
    if debug("X = %d\n", x) != 2 {
        std::process::exit(2);
    }

    if showlist!("The first", "second", "and third items.") != "The first, second, and third items." {
        std::process::exit(3);
    }

    if report(x > y, "x is {} but y is {}", x, y) != 1 {
        std::process::exit(4);
    }
    if report(x < y, "x is {} but y is {}", x, y) != 3 {
        std::process::exit(5);
    }
}

fn debug(args: &str) -> usize {
    1
}

macro_rules! debug {
    ($($arg:tt)*) => {{
        std::macro_export::count_tts!($($arg)*)
    }};
}

macro_rules! showlist {
    ($($item:expr),*) => {
        concat!($(stringify!($item), ", "),*)
    };
}

fn report<T>(condition: T, _args: &str) -> usize
where
    T: std::ops::Not<Output = bool>,
{
    if condition.not() {
        3
    } else {
        1
    }
}

macro_rules! report {
    ($condition:expr, $($arg:tt)*) => {{
        if $condition {
            1
        } else {
            std::macro_export::count_tts!($($arg)*)
        }
    }};
}

macro_rules! count_tts {
    () => (0);
    ($head:tt $($tail:tt)*) => (1 + count_tts!($($tail)*));
}