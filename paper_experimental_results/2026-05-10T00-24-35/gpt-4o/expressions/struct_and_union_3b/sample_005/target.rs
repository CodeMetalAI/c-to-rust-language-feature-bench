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
        U12::S2(_) => 0, // Unreachable in this specific program logic
    }
}

fn main() {
    let mut u = U12::S1(T1 { m: -7 });

    if let U12::S1(ref mut s1) = u {
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
}