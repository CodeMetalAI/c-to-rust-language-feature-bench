use std::process;

macro_rules! max {
    ($a:expr, $b:expr) => {{
        if $a > $b {
            $a
        } else {
            $b
        }
    }};
}

fn post_inc(x: &mut i32) -> i32 {
    let v = *x;
    *x += 1;
    v
}

fn main() {
    let i = 3;
    let j = 5;
    if max!(i, j) != 5 {
        process::exit(1);
    }

    let x = 2.5f64;
    let y = 1.5f64;
    if max!(x, y) != 2.5 {
        process::exit(2);
    }

    let mut k = 1;
    let mut r = max!(post_inc(&mut k), 2);
    if r != 2 {
        process::exit(3);
    }
    if k != 2 {
        process::exit(4);
    }

    let mut m = 3;
    let mut n = 1;
    r = max!(post_inc(&mut m), post_inc(&mut n));
    if r != 3 {
        process::exit(5);
    }
    if m != 4 {
        process::exit(6);
    }
    if n != 2 {
        process::exit(7);
    }
}