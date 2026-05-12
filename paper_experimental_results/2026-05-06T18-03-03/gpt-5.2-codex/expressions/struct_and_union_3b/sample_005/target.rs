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

    let r = f(&mut u);
    if r != -7 {
        std::process::exit(1);
    }
    if u.m != -7 {
        std::process::exit(2);
    }

    std::process::exit(0);
}