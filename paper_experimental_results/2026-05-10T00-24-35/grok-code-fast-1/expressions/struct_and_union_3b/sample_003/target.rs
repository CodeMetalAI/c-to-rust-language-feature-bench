#[derive(Clone, Copy)]
struct T1 {
    m: i32,
}

#[derive(Clone, Copy)]
struct T2 {
    m: i32,
}

fn f(u: &mut i32) -> i32 {
    if *u < 0 {
        *u = -*u;
    }
    *u
}

fn main() {
    let mut u: i32 = 0;

    u = -7; // simulating u.s1.m = -7;
    u = 7;  // simulating u.s2.m = 7;

    if f(&mut u) != -7 {
        std::process::exit(1);
    }
    if u != -7 {
        std::process::exit(2);
    }

    std::process::exit(0);
}