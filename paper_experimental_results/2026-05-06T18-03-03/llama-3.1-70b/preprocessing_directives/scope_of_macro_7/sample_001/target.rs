fn main() {
    let x = 7;
    let y = 3;

    if debug("Flag") != 1 {
        return 1;
    }
    if debug("X = %d\n", x) != 2 {
        return 2;
    }

    if showlist!("The first, second, and third items.") != "The first, second, and third items." {
        return 3;
    }

    if report(x > y, "x is {} but y is {}", x, y) != 1 {
        return 4;
    }
    if report(x < y, "x is {} but y is {}", x, y) != 3 {
        return 5;
    }

    return 0;
}

macro_rules! PP_CAT {
    ($a:expr, $b:expr) => {
        concat!($a, $b)
    };
}

macro_rules! PP_XCAT {
    ($a:expr, $b:expr) => {
        PP_CAT!($a, $b)
    };
}

macro_rules! PP_ARG_N {
    ($($n:expr),*) => {
        count_args!($($n),*)
    };
}

macro_rules! count_args {
    ($($n:expr),*) => {
        <[()]($($n),*)>::LEN
    };
}

macro_rules! PP_NARG {
    ($($n:expr),*) => {
        PP_ARG_N!($($n),*)
    };
}

macro_rules! debug {
    ($($arg:tt)*) => {
        PP_NARG!($($arg)*)
    };
}

macro_rules! showlist {
    ($($arg:tt)*) => {
        stringify!($($arg)*)
    };
}

macro_rules! report {
    ($test:expr, $fmt:expr, $($arg:expr),*) => {
        if $test {
            1
        } else {
            PP_NARG!($($arg),*)
        }
    };
}