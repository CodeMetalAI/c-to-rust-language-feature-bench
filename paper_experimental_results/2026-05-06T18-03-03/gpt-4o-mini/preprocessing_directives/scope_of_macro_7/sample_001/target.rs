fn streq(a: &str, b: &str) -> bool {
    a == b
}

macro_rules! debug {
    ($($arg:tt)*) => {
        count_args!($($arg)*)
    };
}

macro_rules! showlist {
    ($($arg:expr),*) => {
        format!("{:?}", vec![$($arg),*])
    };
}

macro_rules! report {
    ($test:expr, $($arg:tt)*) => {
        if $test {
            1
        } else {
            count_args!($($arg)*)
        }
    };
}

macro_rules! count_args {
    ($($arg:tt)*) => {
        count_args_helper!($($arg)*, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0)
    };
}

macro_rules! count_args_helper {
    ($first:expr, $($rest:expr),*) => {
        count_args_helper!($($rest)*)
    };
    () => {
        0
    };
    ($first:expr) => {
        1
    };
}

fn main() -> i32 {
    let x = 7;
    let y = 3;

    if debug!("Flag") != 1 {
        return 1;
    }
    if debug!("X = {}", x) != 2 {
        return 2;
    }

    if !streq(showlist!("The first", "second", "and third items."), "The first, second, and third items.") {
        return 3;
    }

    if report!(x > y, "x is {} but y is {}", x, y) != 1 {
        return 4;
    }
    if report!(x < y, "x is {} but y is {}", x, y) != 3 {
        return 5;
    }

    0
}