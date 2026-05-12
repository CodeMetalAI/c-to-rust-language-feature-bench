fn main() {
    let x = 7;
    let y = 3;

    if debug("Flag")!= 1 {
        return 1;
    }
    if debug("X = %d\n", x)!= 2 {
        return 2;
    }

    if showlist!("The first, second, and third items.")!= "The first, second, and third items." {
        return 3;
    }

    if report(x > y, "x is {} but y is {}", x, y)!= 1 {
        return 4;
    }
    if report(x < y, "x is {} but y is {}", x, y)!= 3 {
        return 5;
    }

    println!("");
}

macro_rules! PP_CAT {
    ($a:expr, $b:expr) => {{
        concat!($a, $b)
    }};
}

macro_rules! PP_XCAT {
    ($a:expr, $b:expr) => {{
        PP_CAT!($a, $b)
    }};
}

macro_rules! PP_ARG_N {
    ($($arg:expr),*) => {{
        <[()]>::len(&[$(PP_ARG_N!($arg)),*])
    }};
}

macro_rules! PP_NARG_ {
    ($($arg:expr),*) => {{
        PP_ARG_N!($($arg),*)
    }};
}

macro_rules! PP_NARG {
    ($($arg:expr),*) => {{
        PP_NARG_!($($arg),*)
    }};
}

macro_rules! debug {
    ($($arg:expr),*) => {{
        PP_NARG!($($arg),*)
    }};
}

macro_rules! showlist {
    ($($arg:expr),*) => {{
        stringify!($($arg),*)
    }};
}

macro_rules! report {
    ($test:expr, $($arg:expr),*) => {{
        if $test {
            1
        } else {
            PP_NARG!($($arg),*)
        }
    }};
}