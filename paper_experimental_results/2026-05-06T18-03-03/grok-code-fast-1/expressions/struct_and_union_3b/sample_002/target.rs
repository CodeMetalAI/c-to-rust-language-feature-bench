use std::process;

fn f(u: &mut i32) -> i32 {
    if *u < 0 {
        *u = -*u;
    }
    *u
}

fn main() {
    let mut u: i32 = -7;
    u = 7;
    if f(&mut u) != -7 {
        process::exit(1);
    }
    if u != -7 {
        process::exit(2);
    }
    process::exit(0);
}