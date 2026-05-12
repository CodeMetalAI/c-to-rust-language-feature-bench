#[repr(C)]
struct T1 {
    m: i32,
}

#[repr(C)]
struct T2 {
    m: i32,
}

#[repr(C)]
union U12 {
    s1: T1,
    s2: T2,
}

fn f(u: &mut U12) -> i32 {
    if unsafe { u.s1.m } < 0 {
        unsafe { u.s2.m = -u.s2.m };
    }
    unsafe { u.s1.m }
}

fn main() {
    let mut u = U12 { s1: T1 { m: -7 } };
    unsafe { u.s2.m = 7 };

    if f(&mut u) != -7 {
        return;
    }
    if unsafe { u.s2.m } != -7 {
        return;
    }
}