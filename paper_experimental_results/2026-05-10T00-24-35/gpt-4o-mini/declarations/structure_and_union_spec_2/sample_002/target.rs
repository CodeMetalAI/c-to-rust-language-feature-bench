use std::alloc::{alloc, dealloc, Layout};

struct S {
    n: i32,
    d: [f64; 0], // zero-sized array
}

struct SS {
    n: i32,
}

fn main() {
    // Checking size conditions
    if std::mem::size_of::<S>() < std::mem::size_of::<SS>() {
        std::process::exit(1);
    }

    if std::mem::offset_of!(S, d) != std::mem::size_of::<S>() {
        std::process::exit(1);
    }

    let layout1 = Layout::from_size_align(std::mem::size_of::<S>() + std::mem::size_of::<f64>() * 8, std::mem::align_of::<f64>()).unwrap();
    let layout2 = Layout::from_size_align(std::mem::size_of::<S>() + std::mem::size_of::<f64>() * 5, std::mem::align_of::<f64>()).unwrap();

    // Allocating memory
    let s1 = unsafe { alloc(layout1) as *mut S };
    let s2 = unsafe { alloc(layout2) as *mut S };

    if s1.is_null() || s2.is_null() {
        std::process::exit(1);
    }

    unsafe {
        (*s1).d[0] = 42.0;
        (*s2).d[0] = 24.0;

        if (*s1).d[0] != 42.0 || (*s2).d[0] != 24.0 {
            std::process::exit(1);
        }

        dealloc(s1 as *mut u8, layout1);
        dealloc(s2 as *mut u8, layout2);
    }

    std::process::exit(0);
}