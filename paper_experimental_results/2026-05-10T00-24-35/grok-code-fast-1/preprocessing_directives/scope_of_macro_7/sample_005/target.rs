macro_rules! pp_rseq_n {
    () => {
        10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0
    };
}

macro_rules! pp_arg_n {
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $n:expr $(,)?) => {
        $n
    };
}

macro_rules! pp_narg_ {
    ($($args:expr),* $(,)?) => {
        pp_arg_n!($($args,)* pp_rseq_n!())
    };
}

macro_rules! pp_narg {
    ($($args:expr),* $(,)?) => {
        pp_narg_!($($args,)* pp_rseq_n!())
    };
}

macro_rules! debug {
    ($($args:expr),* $(,)?) => {
        pp_narg!($($args),*)
    };
}

macro_rules! showlist {
    ($($args:tt)*) => {
        stringify!($($args)*)
    };
}

macro_rules! report {
    ($test:expr, $($args:expr),* $(,)?) => {
        if $test {
            1
        } else {
            pp_narg!($($args),*)
        }
    };
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn main() {
    let x: i32 = 7;
    let y: i32 = 3;

    if debug!("Flag") != 1 {
        std::process::exit(1);
    }
    if debug!("X = %d\n", x) != 2 {
        std::process::exit(2);
    }

    if !streq(
        showlist!(The first, second, and third items.),
        "The first, second, and third items.",
    ) {
        std::process::exit(3);
    }

    if report!(x > y, "x is %d but y is %d", x, y) != 1 {
        std::process::exit(4);
    }
    if report!(x < y, "x is %d but y is %d", x, y) != 3 {
        std::process::exit(5);
    }

    std::process::exit(0);
}