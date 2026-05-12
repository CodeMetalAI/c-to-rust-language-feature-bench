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
    ($first:expr, $($rest:expr),*, $n:expr, $($args:expr),*) => {
        $n
    };
    ($first:expr, $($rest:expr),*, $n:expr) => {
        $n
    };
}

macro_rules! pp_rseq_n {
    () => {
        10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0
    };
}

macro_rules! pp_narg {
    () => {
        pp_arg_n!(,,,,,,,,,, 0, __VA_ARGS__)
    };
    ($($rest:expr),*) => {
        pp_arg_n!(, $($rest),*, 10, __VA_ARGS__)
    };
}

macro_rules! debug {
    ($($rest:expr),*) => {
        pp_narg!($($rest),*)
    };
}

macro_rules! showlist {
    ($($rest:expr),*) => {
        concat!($($rest),*)
    };
}

macro_rules! report {
    ($test:expr, $($rest:expr),*) => {
        if $test {
            1
        } else {
            pp_narg!($($rest),*)
        }
    };
}

fn streq(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).all(|(a, b)| a == b)
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