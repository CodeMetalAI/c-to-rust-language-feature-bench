macro_rules! arg_n {
    ($a1:tt, $a2:tt, $a3:tt, $a4:tt, $a5:tt, $a6:tt, $a7:tt, $a8:tt, $a9:tt, $a10:tt, $n:tt, $($rest:tt)*) => {$n};
}

macro_rules! rseq_n {
    () => {10,9,8,7,6,5,4,3,2,1,0};
}

macro_rules! narg_ {
    ($($args:tt)*) => { arg_n!($($args)*) };
}

macro_rules! narg {
    ($($args:tt)*) => { narg_!($($args)* , rseq_n!()) };
}

macro_rules! debug {
    ($($args:tt)*) => { narg!($($args)*) };
}

macro_rules! showlist {
    ($($args:tt)*) => { stringify!($($args)*) };
}

macro_rules! report {
    ($test:expr, $($args:tt)*) => { if $test {1} else { narg!($($args)*) } };
}

fn streq(a: &str, b: &str) -> i32 {
    if a == b { 1 } else { 0 }
}

fn main() {
    let x = 7i32;
    let y = 3i32;

    if debug!("Flag") != 1 {
        std::process::exit(1);
    }
    if debug!("X = %d\n", x) != 2 {
        std::process::exit(2);
    }

    if streq(showlist!(The first, second, and third items.), "The first, second, and third items.") == 0 {
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