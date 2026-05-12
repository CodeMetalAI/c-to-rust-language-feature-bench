struct T1 {
    m: i32,
}

struct T2 {
    m: i32,
}

union U12 {
    s1: T1,
    s2: T2,
}

fn f(u: &mut U12) -> i32 {
    unsafe {
        if (*u).s1.m < 0 {
            (*u).s2.m = -(*u).s2.m;
        }
        (*u).s1.m
    }
}

fn main() -> i32 {
    let mut u = U12 { s1: T1 { m: -7 } };
    unsafe { u.s2 = T2 { m: 7 } };

    if f(&mut u) != -7 {
        return 1;
    }
    if unsafe { u.s2.m } != -7 {
        return 2;
    }

    0
}