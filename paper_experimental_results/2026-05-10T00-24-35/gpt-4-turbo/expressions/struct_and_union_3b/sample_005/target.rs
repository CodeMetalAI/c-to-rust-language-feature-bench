fn main() {
    let mut u = U12 { s1: T1 { m: -7 } };

    // Mimicking the union's behavior where s1 and s2 share the same memory location
    unsafe {
        u.s2.m = 7;
    }

    let result = f(&mut u);
    if result != -7 {
        std::process::exit(1);
    }

    // Check the shared memory location after function modifies it
    if unsafe { u.s2.m } != -7 {
        std::process::exit(2);
    }
}

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
        if u.s1.m < 0 {
            u.s2.m = -u.s2.m;
        }
        u.s1.m
    }
}