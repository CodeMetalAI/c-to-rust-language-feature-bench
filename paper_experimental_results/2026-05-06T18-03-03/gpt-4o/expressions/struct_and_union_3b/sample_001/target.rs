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
    fn get_s1(&self) -> Option<&T1> {
        if let U12::S1(ref s1) = *self {
            Some(s1)
        } else {
            None
        }
    }

    fn get_s2_mut(&mut self) -> Option<&mut T2> {
        if let U12::S2(ref mut s2) = *self {
            Some(s2)
        } else {
            None
        }
    }
}

fn f(u: &mut U12) -> i32 {
    if let Some(s1) = u.get_s1() {
        if s1.m < 0 {
            if let Some(s2) = u.get_s2_mut() {
                s2.m = -s2.m;
            }
        }
        s1.m
    } else {
        0
    }
}

fn main() {
    let mut u = U12::S1(T1 { m: -7 });

    // Simulate the union behavior by switching to S2
    if let U12::S1(s1) = u {
        u = U12::S2(T2 { m: s1.m.abs() });
    }

    if f(&mut u) != -7 {
        std::process::exit(1);
    }

    if let U12::S2(ref s2) = u {
        if s2.m != -7 {
            std::process::exit(2);
        }
    }
}