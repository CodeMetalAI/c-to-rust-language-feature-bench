macro_rules! pp_cat {
    ($a:ident, $b:ident) => { concat_idents!($a, $b) }
}

macro_rules! pp_xcat {
    ($a:ident, $b:ident) => { pp_cat!($a, $b) }
}

macro_rules! pp_arg_n {
    ($($args:tt)*) => { 
        $(
            0;
        )*
        10
    };
}

macro_rules! pp_rseq_n {
    () => { 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0 }
}

macro_rules! pp_narg_ {
    ($($args:tt)*) => { pp_arg_n!($($args)*) };
}

macro_rules! pp_narg {
    ($($args:tt)*) => { pp_narg_!($($args), pp_rseq_n!()) };
}

macro_rules! debug {
    ($($args:tt)*) => { pp_narg!($($args)*) };
}

macro_rules! showlist {
    ($($args:tt)*) => { stringify!($($args)*) };
}

macro_rules! report {
    ($test:expr, $($args:tt)*) => { if $test { 1 } else { pp_narg!($($args)*) } };
}

fn streq(a: &str, b: &str) -> bool {
    a == b
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

    if !streq(showlist!(The first, second, and third items.), "The first, second, and third items.") {
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