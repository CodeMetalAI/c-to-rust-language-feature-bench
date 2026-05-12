use std::mem::{size_of, size_of_val};
use std::ptr::{null_mut, NonNull};

fn main() {
    #[repr(C)]
    struct S {
        n: i32,
        d: [f64; 0],
    }

    #[repr(C)]
    struct SS {
        n: i32,
    }

    if size_of::<S>() < size_of::<SS>() {
        std::process::exit(1);
    }

    if size_of_val(&S { n: 0, d: [] }) != size_of::<S>() {
        std::process::exit(1);
    }

    let s1 = unsafe {
        let ptr = libc::malloc(size_of::<S>() + size_of::<f64>() * 8) as *mut S;
        if ptr.is_null() {
            std::process::exit(1);
        }
        NonNull::new(ptr).unwrap()
    };

    let s2 = unsafe {
        let ptr = libc::malloc(size_of::<S>() + size_of::<f64>() * 5) as *mut S;
        if ptr.is_null() {
            std::process::exit(1);
        }
        NonNull::new(ptr).unwrap()
    };

    unsafe {
        (*s1.as_ptr()).d[0] = 42.0;
        (*s2.as_ptr()).d[0] = 24.0;
    }

    if unsafe { (*s1.as_ptr()).d[0] != 42.0 || (*s2.as_ptr()).d[0] != 24.0 } {
        std::process::exit(1);
    }

    unsafe {
        libc::free(s1.as_ptr() as *mut libc::c_void);
        libc::free(s2.as_ptr() as *mut libc::c_void);
    }
}