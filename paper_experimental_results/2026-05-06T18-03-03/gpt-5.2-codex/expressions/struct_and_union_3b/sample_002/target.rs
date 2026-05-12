use std::process::exit;

struct U12 {
    m: i32,
}

fn f(u: &mut U12) -> i32 {
    if u.m < 0 {
        u.m = -u.m;
    }
    u.m
}

fn main() {
    let mut u = U12 { m: 0 };

    u.m = -7;
    u.m = 7;

    if f(&mut u) != -7 {
        exit(1);
    }
    if u.m != -7 {
        exit(2);
    }
}