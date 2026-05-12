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

    let s1 = unsafe { Box::from_raw(Box::into_raw(vec![0.0; 8].into_boxed_slice()) as *mut [f64]) };
    let s2 = unsafe { Box::from_raw(Box::into_raw(vec![0.0; 5].into_boxed_slice()) as *mut [f64]) };

    s1[0] = 42.0;
    s2[0] = 24.0;

    if s1[0] != 42.0 || s2[0] != 24.0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}