fn main() {
    let x = 7;
    let y = 3;

    // debug macro counts arguments
    if debug!("Flag") != 1 {
        return;
    }
    if debug!("X = %d\n", x) != 2 {
        return;
    }

    // showlist macro concatenates arguments into a string literal
    if !showlist!(The first, second, and third items.)
        .eq("The first, second, and third items.")
    {
        return;
    }

    // report macro returns 1 if test is true, otherwise counts arguments
    if report!(x > y, "x is {} but y is {}", x, y) != 1 {
        return;
    }
    if report!(x < y, "x is {} but y is {}", x, y) != 3 {
        return;
    }
}

macro_rules! debug {
    ($($arg:tt)*) => {
        count_args!($($arg)*)
    };
}

macro_rules! showlist {
    ($($arg:tt)*) => {
        stringify!($($arg)*)
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
    () => { 0 };
    ($first:tt $($rest:tt)*) => { 1 + count_args!($($rest)*) };
}