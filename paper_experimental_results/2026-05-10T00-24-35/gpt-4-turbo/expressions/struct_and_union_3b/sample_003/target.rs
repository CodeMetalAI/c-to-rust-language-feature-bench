fn main() {
    #[derive(Debug)]
    struct T1 {
        m: i32,
    }

    #[derive(Debug)]
    struct T2 {
        m: i32,
    }

    union U12 {
        s1: T1,
        s2: T2,
    }

    fn f(u: &mut U12) -> i32 {
        unsafe {
            if u.s1.m < 0 {
                u.s2.m = -u.s2.m;
            }
            u.s1.m
        }
    }

    let mut u = U12 { s1: T1 { m: -7 } };

    unsafe {
        // This sets `s2.m` to `7`, effectively overwriting `s1.m` due to union semantics
        u.s2.m = 7;

        if f(&mut u) != -7 {
            std::process::exit(1);
        }

        if u.s2.m != -7 {
            std::process::exit(2);
        }
    }
}