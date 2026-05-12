use std::mem::MaybeUninit;

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
    unsafe {
        if u.s1.m < 0 {
            u.s2.m = -u.s2.m;
        }
        u.s1.m
    }
}

fn main() {
    let mut u = MaybeUninit::<U12>::uninit();
    let u_ptr = u.as_mut_ptr();
    
    unsafe {
        (*u_ptr).s1 = T1 { m: -7 };
        (*u_ptr).s2 = T2 { m: 7 };
    }
    
    let mut u = unsafe { u.assume_init() };
    
    if f(&mut u) != -7 {
        std::process::exit(1);
    }
    
    let s2_m = unsafe { u.s2.m };
    if s2_m != -7 {
        std::process::exit(2);
    }
    
    std::process::exit(0);
}