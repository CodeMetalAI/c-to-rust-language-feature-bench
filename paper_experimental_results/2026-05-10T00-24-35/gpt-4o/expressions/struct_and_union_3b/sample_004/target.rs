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
    fn get_s1(&self) -> &T1 {
        if let U12::S1(ref s1) = self {
            s1
        } else {
            panic!("Union does not currently hold T1");
        }
    }

    fn get_mut_s2(&mut self) -> &mut T2 {
        if let U12::S2(ref mut s2) = self {
            s2
        } else {
            panic!("Union does not currently hold T2");
        }
    }
}

fn f(u: &mut U12) -> i32 {
    if let U12::S1(ref s1) = *u {
        if s1.m < 0 {
            if let U12::S2(ref mut s2) = *u {
                s2.m = -s2.m;
            }
        }
        s1.m
    } else {
        panic!("Union does not currently hold T1");
    }
}

fn main() -> i32 {
    let mut u = U12::S1(T1 { m: -7 });

    // As Rust enums cannot encapsulate two values simultaneously like C unions,
    // we change the enum variant to S2 and set its value.
    // This ensures that later access to S2 is valid.
    u = U12::S2(T2 { m: 7 });

    if f(&mut u) != -7 {
        return 1;
    }
    if let U12::S2(ref s2) = u {
        if s2.m != -7 {
            return 2;
        }
    } else {
        return 2;
    }

    0
}