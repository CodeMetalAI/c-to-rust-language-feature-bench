struct T1 {
    m: i32,
}

struct T2 {
    m: i32,
}

union U12 {
    t1: T1,
    t2: T2,
}

fn f(u: &U12) -> i32 {
    if u.t1.m < 0 {
        u.t2.m = -u.t2.m;
    }
    u.t1.m
}

fn main() {
    let mut u = U12 { t1: T1 { m: -7 }, t2: T2 { m: 7 } };

    if f(&u) != -7 {
        std::process::exit(1);
    }
    if u.t2.m != std::mem::MaybeUninit::<i32>::uninit().assume_init() {
        std::process::exit(2);
    }

    std::process::exit(0);
}