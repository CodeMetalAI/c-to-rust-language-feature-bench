struct T1 {
    m: i32,
}

struct T2 {
    m: i32,
}

enum U12 {
    S1(T1),
    S2(T2),
}

impl U12 {
    fn as_s1(&self) -> &T1 {
        match self {
            U12::S1(ref s1) => s1,
            U12::S2(_) => panic!("Tried to access S1, but it's currently S2"),
        }
    }
    
    fn as_s2_mut(&mut self) -> &mut T2 {
        match self {
            U12::S2(ref mut s2) => s2,
            U12::S1(_) => panic!("Tried to access S2, but it's currently S1"),
        }
    }
}

fn f(u: &mut U12) -> i32 {
    let m = u.as_s1().m;
    if m < 0 {
        let s2 = u.as_s2_mut();
        s2.m = -s2.m;
    }
    m
}

fn main() {
    let mut u = U12::S1(T1 { m: -7 });
    if let U12::S1(_) = u {
        u = U12::S2(T2 { m: 7 });
    }

    if f(&mut u) != -7 {
        std::process::exit(1);
    }

    if let U12::S2(ref s2) = u {
        if s2.m != -7 {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}