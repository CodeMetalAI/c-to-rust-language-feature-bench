fn f(u: &mut U12) -> i32 {
    if u.s1.m < 0 {
        u.s2.m = -u.s2.m;
    }
    u.s1.m
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct T1 {
    m: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct T2 {
    m: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
union U12 {
    s1: T1,
    s2: T2,
}

fn main() {
    let mut u = U12 { s1: T1 { m: -7 } };

    u.s2.m = 7;

    if f(&mut u)!= -7 {
        std::process::exit(1);
    }
    if u.s2.m!= -7 {
        std::process::exit(2);
    }

    std::process::exit(0);
}