fn main() {
    let x = 7;
    let y = 3;

    if count_args!("Flag")!= 1 {
        return 1;
    }
    if count_args!("X = %d\n", x)!= 2 {
        return 2;
    }

    if stringify!(The first, second, and third items.)!= "The first, second, and third items." {
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

macro_rules! count_args {
    ($($arg:tt)*) => {{
        let mut count = 0;
        $(
            count += 1;
        )*
        count
    }};
}

macro_rules! stringify {
    ($($arg:tt)*) => {
        stringify!($($arg)*)
    };
}

macro_rules! report {
    ($test:expr, $format:expr, $($arg:tt)*) => {{
        if $test {
            1
        } else {
            count_args!($($arg)*)
        }
    }};
}