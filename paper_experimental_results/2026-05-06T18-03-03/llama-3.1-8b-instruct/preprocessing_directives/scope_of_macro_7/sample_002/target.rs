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
    ($(_arg:tt)*) => {
        0
    };
    ($first:ident, $($rest:ident),*) => {
        $rest
    };
}

macro_rules! pp_rseq_n {
    () => {
        10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0
    };
}

macro_rules! pp_narg {
    () => {
        pp_arg_n!()
    };
    ($first:ident, $($rest:ident),*) => {
        pp_arg_n!($first, $($rest),*)
    };
}

macro_rules! debug {
    ($($arg:tt)*) => {
        pp_narg!($($arg)*)
    };
}

macro_rules! showlist {
    ($($arg:tt)*) => {
        concat!($($arg)*)
    };
}

fn streq(a: &str, b: &str) -> bool {
    let mut a = a.as_bytes();
    let mut b = b.as_bytes();
    while !a.is_empty() && !b.is_empty() {
        if *a != *b {
            return false;
        }
        a = &a[1..];
        b = &b[1..];
    }
    a.is_empty() && b.is_empty()
}

fn main() {
    let x = 7;
    let y = 3;

    if debug!("Flag") != 1 {
        return 1;
    }
    if debug!("X = {}", x) != 2 {
        return 2;
    }

    if !streq(showlist!("The first, second, and third items."), "The first, second, and third items.") {
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

fn report(test: bool, $($arg:tt)*) -> usize {
    if test {
        1
    } else {
        debug!($($arg)*)
    }
}