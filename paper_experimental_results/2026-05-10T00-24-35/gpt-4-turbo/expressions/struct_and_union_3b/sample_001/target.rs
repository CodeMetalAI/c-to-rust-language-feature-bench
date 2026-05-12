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
        U12::S1(s1) if s1.m < 0 => {
            if let U12::S2(s2) = u {
                s2.m = -s2.m;
            }
        }
        _ => {}
    }

    match u {
        U12::S1(s1) => s1.m,
        U12::S2(s2) => s2.m,
    }
}

fn main() {
    let mut u = U12::S1(T1 { m: -7 });
    match &mut u {
        U12::S2(s2) => s2.m = 7,
        _ => {}
    }

    if f(&mut u) != -7 {
        std::process::exit(1);
    }
    match u {
        U12::S2(s2) if s2.m != -7 => std::process::exit(2),
        _ => {}
    }
}