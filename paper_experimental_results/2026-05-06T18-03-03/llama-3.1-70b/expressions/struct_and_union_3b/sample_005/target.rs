#[derive(Debug, Default)]
struct T1 {
    m: i32,
}

#[derive(Debug, Default)]
struct T2 {
    m: i32,
}

#[derive(Debug, Default)]
union U12 {
    s1: T1,
    s2: T2,
}

fn f(u: &mut U12) -> i32 {
    if unsafe { (*u).s1.m } < 0 {
        unsafe { (*u).s2.m } = -unsafe { (*u).s2.m };
    }
    unsafe { (*u).s1.m }
}

fn main() {
    let mut u = U12::default();

    unsafe {
        (*u).s1.m = -7;
        (*u).s2.m = 7;
    }

    if f(&mut u)!= -7 {
        std::process::exit(1);
    }
    if unsafe { (*u).s2.m }!= -7 {
        std::process::exit(2);
    }

    std::process::exit(0);
}