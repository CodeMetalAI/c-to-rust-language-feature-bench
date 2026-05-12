use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

struct S {
    n: i32,
    d: [f64; 0], // Placeholder for flexible array member
}

struct SS {
    n: i32,
}

fn main() {
    // Check sizes
    if mem::size_of::<S>() < mem::size_of::<SS>() {
        std::process::exit(1);
    }

    // Check offset
    if mem::offset_of!(S, d) != mem::size_of::<S>() {
        std::process::exit(1);
    }

    // Allocate memory for S with space for the array `d`
    let s1_layout = Layout::from_size_align(mem::size_of::<S>() + mem::size_of::<f64>() * 8, mem::align_of::<S>()).unwrap();
    let s2_layout = Layout::from_size_align(mem::size_of::<S>() + mem::size_of::<f64>() * 5, mem::align_of::<S>()).unwrap();

    let s1_ptr = unsafe { alloc(s1_layout) as *mut S };
    let s2_ptr = unsafe { alloc(s2_layout) as *mut S };

    if s1_ptr.is_null() || s2_ptr.is_null() {
        unsafe {
            if !s1_ptr.is_null() {
                dealloc(s1_ptr as *mut u8, s1_layout);
            }
            if !s2_ptr.is_null() {
                dealloc(s2_ptr as *mut u8, s2_layout);
            }
        }
        std::process::exit(1);
    }

    unsafe {
        (*s1_ptr).d[0] = 42.0;
        (*s2_ptr).d[0] = 24.0;

        if (*s1_ptr).d[0] != 42.0 || (*s2_ptr).d[0] != 24.0 {
            dealloc(s1_ptr as *mut u8, s1_layout);
            dealloc(s2_ptr as *mut u8, s2_layout);
            std::process::exit(1);
        }

        dealloc(s1_ptr as *mut u8, s1_layout);
        dealloc(s2_ptr as *mut u8, s2_layout);
    }

    std::process::exit(0);
}