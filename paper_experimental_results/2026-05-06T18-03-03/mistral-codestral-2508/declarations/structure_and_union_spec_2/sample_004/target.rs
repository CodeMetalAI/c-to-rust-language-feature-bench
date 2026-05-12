use std::mem;

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

    if mem::size_of::<S>() < mem::size_of::<SS>() {
        std::process::exit(1);
    }

    let offset = unsafe { &(*(0 as *const S)).d as *const _ as usize };
    if offset != mem::size_of::<S>() {
        std::process::exit(1);
    }

    let s1 = unsafe { Box::from_raw(libc::malloc(mem::size_of::<S>() + 8 * mem::size_of::<f64>()) as *mut S) };
    let s2 = unsafe { Box::from_raw(libc::malloc(mem::size_of::<S>() + 5 * mem::size_of::<f64>()) as *mut S) };

    if s1.is_null() || s2.is_null() {
        std::process::exit(1);
    }

    unsafe {
        (*s1).d[0] = 42.0;
        (*s2).d[0] = 24.0;
    }

    if unsafe { (*s1).d[0] } != 42.0 || unsafe { (*s2).d[0] } != 24.0 {
        std::process::exit(1);
    }

    unsafe {
        libc::free(s1 as *mut _ as *mut libc::c_void);
        libc::free(s2 as *mut _ as *mut libc::c_void);
    }
}