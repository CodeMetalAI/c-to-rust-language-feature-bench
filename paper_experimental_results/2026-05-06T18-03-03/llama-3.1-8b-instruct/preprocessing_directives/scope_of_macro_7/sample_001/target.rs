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
    ($(_1:tt, $(_2:tt,)* $(_3:tt,)* $(_4:tt,)* $(_5:tt,)* $(_6:tt,)* $(_7:tt,)* $(_8:tt,)* $(_9:tt,)* $(_10:tt,)* $N:expr, $($rest:tt)*) => {
        $N
    };
}

macro_rules! pp_rseq_n {
    () => {
        10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0
    };
}

macro_rules! pp_narg {
    ($($rest:tt)*) => {
        pp_arg_n!($($rest)++, pp_rseq_n!())
    };
}

macro_rules! debug {
    ($($rest:tt)*) => {
        pp_narg!($($rest)++)
    };
}

macro_rules! showlist {
    ($($rest:tt)*) => {
        #!($($rest)*)
    };
}

fn streq(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).all(|(c1, c2)| c1 == c2)
}

fn main() {
    let x = 7;
    let y = 3;

    if debug!("Flag") != 1 {
        return 1;
    }
    if debug!("X = {}{}", x, "\n") != 2 {
        return 2;
    }

    if !streq(showlist!("The first, second, and third items."), "The first, second, and third items.") {
        return 3;
    }

    if report!(x > y, "x is {} but y is {}", x, y) != 1 {
        return 4;
    }
    if report!(x < y, "x is {} but y is {}", x, y) != 3 {
        return 5;
    }

    return 0;
}