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

fn debug<T: std::fmt::Display>(fmt: &str, arg: T) -> usize {
    2
}

macro_rules! showlist {
    ($($item:expr),*) => {
        concat!($(stringify!($item), ", "),*)
    };
}

fn report<T: std::fmt::Display>(test: bool, fmt: &str, x: T, y: T) -> usize {
    if test {
        1
    } else {
        3
    }
}