#[derive(Debug, Copy, Clone)]
struct t1 {
    m: i32,
}
#[derive(Debug, Copy, Clone)]
struct t2 {
    m: i32,
}

#[derive(Debug, Copy, Clone)]
union u12 {
    s1: t1,
    s2: t2,
}

fn f(u: &mut u12) -> i32 {
    if u.s1.m < 0 {
        u.s2.m = -u.s2.m;
    }
    u.s1.m
}

fn main() {
    let mut u = u12 {
        s1: t1 { m: -7 },
        s2: t2 { m: 7 },
    };

    if f(&mut u) != -7 {
        std::process::exit(1);
    }
    if u.s2.m != -7 {
        std::process::exit(2);
    }

    std::process::exit(0);
}