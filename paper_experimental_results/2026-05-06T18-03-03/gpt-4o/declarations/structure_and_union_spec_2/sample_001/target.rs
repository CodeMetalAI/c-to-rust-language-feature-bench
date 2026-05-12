use std::mem;
use std::ptr;

struct S {
    n: i32,
    d: [f64; 0],
}

struct Ss {
    n: i32,
}

fn main() {
    if mem::size_of::<S>() < mem::size_of::<Ss>() {
        std::process::exit(1);
    }

    if mem::size_of::<S>() != mem::size_of::<i32>() {
        std::process::exit(1);
    }

    let s1_size = mem::size_of::<S>() + mem::size_of::<f64>() * 8;
    let s2_size = mem::size_of::<S>() + mem::size_of::<f64>() * 5;

    let s1 = unsafe { libc::malloc(s1_size) as *mut S };
    let s2 = unsafe { libc::malloc(s2_size) as *mut S };

    if s1.is_null() || s2.is_null() {
        std::process::exit(1);
    }

    unsafe {
        (*s1).d.as_mut_ptr().write(42.0);
        (*s2).d.as_mut_ptr().write(24.0);

        if (*s1).d[0] != 42.0 || (*s2).d[0] != 24.0 {
            std::process::exit(1);
        }

        libc::free(s1 as *mut libc::c_void);
        libc::free(s2 as *mut libc::c_void);
    }
}