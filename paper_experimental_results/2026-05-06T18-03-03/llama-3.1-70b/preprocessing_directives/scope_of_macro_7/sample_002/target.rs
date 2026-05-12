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

    return 0;
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
    () => (0);
    ($arg:expr) => (1);
    ($arg:expr, $($rest:expr),*) => (1 + count_args!($($rest),*));
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}