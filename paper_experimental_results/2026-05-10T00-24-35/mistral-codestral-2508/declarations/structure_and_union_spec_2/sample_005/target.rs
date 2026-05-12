use std::mem::{size_of, align_of};
use std::alloc::{alloc, dealloc, Layout};

fn main() {
    struct S {
        n: i32,
        d: [f64; 0],
    }

    struct SS {
        n: i32,
    }

    if size_of::<S>() < size_of::<SS>() {
        std::process::exit(1);
    }

    if std::mem::offset_of!(S, d) != size_of::<S>() {
        std::process::exit(1);
    }

    let layout_s1 = Layout::from_size_align(size_of::<S>() + size_of::<f64>() * 8, align_of::<S>()).unwrap();
    let layout_s2 = Layout::from_size_align(size_of::<S>() + size_of::<f64>() * 5, align_of::<S>()).unwrap();

    let s1 = unsafe { alloc(layout_s1) } as *mut S;
    let s2 = unsafe { alloc(layout_s2) } as *mut S;

    if s1.is_null() || s2.is_null() {
        std::process::exit(1);
    }

    unsafe {
        (*s1).d[0] = 42.0;
        (*s2).d[0] = 24.0;

        if (*s1).d[0] != 42.0 || (*s2).d[0] != 24.0 {
            std::process::exit(1);
        }
    }

    unsafe {
        dealloc(s1 as *mut u8, layout_s1);
        dealloc(s2 as *mut u8, layout_s2);
    }
}