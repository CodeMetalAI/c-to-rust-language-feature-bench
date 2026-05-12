macro_rules! pp_cat {
    ($a:expr, $b:expr) => {
        concat!($a, $b)
    };
}

macro_rules! pp_xcat {
    ($a:expr, $b:expr) => {
        pp_cat!($a, $b)
    };
}

macro_rules! pp_arg_n {
    ($($rest:tt)*) => {
        0
    };
    ($first:tt, $($rest:tt)*) => {
        1
    };
    ($first:tt, $second:tt, $($rest:tt)*) => {
        2
    };
    ($first:tt, $second:tt, $third:tt, $($rest:tt)*) => {
        3
    };
    ($first:tt, $second:tt, $third:tt, $fourth:tt, $($rest:tt)*) => {
        4
    };
    ($first:tt, $second:tt, $third:tt, $fourth:tt, $fifth:tt, $($rest:tt)*) => {
        5
    };
    ($first:tt, $second:tt, $third:tt, $fourth:tt, $fifth:tt, $sixth:tt, $($rest:tt)*) => {
        6
    };
    ($first:tt, $second:tt, $third:tt, $fourth:tt, $fifth:tt, $sixth:tt, $seventh:tt, $($rest:tt)*) => {
        7
    };
    ($first:tt, $second:tt, $third:tt, $fourth:tt, $fifth:tt, $sixth:tt, $seventh:tt, $eighth:tt, $($rest:tt)*) => {
        8
    };
    ($first:tt, $second:tt, $third:tt, $fourth:tt, $fifth:tt, $sixth:tt, $seventh:tt, $eighth:tt, $ninth:tt, $($rest:tt)*) => {
        9
    };
    ($first:tt, $second:tt, $third:tt, $fourth:tt, $fifth:tt, $sixth:tt, $seventh:tt, $eighth:tt, $ninth:tt, $tenth:tt, $($rest:tt)*) => {
        10
    };
}

macro_rules! pp_rseq_n {
    () => {
        10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0
    };
}

macro_rules! pp_narg {
    () => {
        0
    };
    ($first:tt, $($rest:tt)*) => {
        pp_arg_n!($first, $($rest)*)
    };
}

macro_rules! debug {
    ($($rest:tt)*) => {
        pp_narg!($($rest)*)
    };
}

macro_rules! showlist {
    ($($rest:tt)*) => {
        concat!("", $($rest)*)
    };
}

macro_rules! report {
    ($test:expr, $($rest:tt)*) => {
        if $test {
            1
        } else {
            pp_narg!($($rest)*)
        }
    };
}

fn streq(a: &str, b: &str) -> bool {
    let mut a = a.chars();
    let mut b = b.chars();
    while let (Some(a), Some(b)) = (a.next(), b.next()) {
        if a!= b {
            return false;
        }
    }
    a.next() == b.next()
}

fn main() {
    let x = 7;
    let y = 3;

    if debug!("Flag")!= 1 {
        return 1;
    }
    if debug!("X = {}{}", x, "\n")!= 2 {
        return 2;
    }

    if!streq(showlist!("The first, second, and third items."), "The first, second, and third items.") {
        return 3;
    }

    if report!(x > y, "x is {} but y is {}", x, y)!= 1 {
        return 4;
    }
    if report!(x < y, "x is {} but y is {}", x, y)!= 3 {
        return 5;
    }

    return 0;
}