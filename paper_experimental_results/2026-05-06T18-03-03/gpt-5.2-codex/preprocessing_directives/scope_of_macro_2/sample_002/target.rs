use std::process;

fn post_inc(x: &mut i32) -> i32 {
    let old = *x;
    *x += 1;
    old
}

macro_rules! c_max {
    ($a:expr, $b:expr) => {{
        let a_val = $a;
        let b_val = $b;
        if a_val > b_val { $a } else { $b }
    }};
}

fn main() {
    let i = 3;
    let j = 5;
    if c_max!(i, j) != 5 {
        process::exit(1);
    }

    let x = 2.5_f64;
    let y = 1.5_f64;
    if c_max!(x, y) != 2.5_f64 {
        process::exit(2);
    }

    let mut k = 1;
    let r = c_max!(post_inc(&mut k), 2);
    if r != 2 {
        process::exit(3);
    }
    if k != 2 {
        process::exit(4);
    }

    let mut m = 3;
    let mut n = 1;
    let r = c_max!(post_inc(&mut m), post_inc(&mut n));
    if r != 3 {
        process::exit(5);
    }
    if m != 4 {
        process::exit(6);
    }
    if n != 2 {
        process::exit(7);
    }

    process::exit(0);
}