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

fn f(u: &mut U12) -> i32 {
    match u {
        U12::S1(s1) => {
            if s1.m < 0 {
                if let U12::S2(s2) = u {
                    s2.m = -s2.m;
                }
            }
            s1.m
        }
        U12::S2(s2) => s2.m, // This arm will never be used in this specific code
    }
}

fn main() -> i32 {
    let mut u = U12::S1(T1 { m: -7 });

    match &mut u {
        U12::S1(_) => {
            u = U12::S2(T2 { m: 7 });
        }
        U12::S2(_) => {
            // This arm will not be used based on initialization above
        }
    }

    if f(&mut u) != -7 {
        return 1;
    }
    if let U12::S2(s2) = u {
        if s2.m != -7 {
            return 2;
        }
    }

    0
}