use std::alloc::{alloc, dealloc, Layout};
use std::mem::{align_of, size_of};

#[repr(C)]
struct S {
    n: i32,
    d: [f64; 0],
}

#[repr(C)]
struct Ss {
    n: i32,
}

fn main() {
    // Check size constraints
    if size_of::<S>() < size_of::<Ss>() {
        std::process::exit(1);
    }

    // Check offset of d field
    let offset_d = unsafe { &(*(std::ptr::null() as *const S)).d as *const _ as usize };
    if offset_d != size_of::<S>() {
        std::process::exit(1);
    }

    // Allocate memory for s1 (8 doubles)
    let s1_size = size_of::<S>() + size_of::<f64>() * 8;
    let s1_layout = Layout::from_size_align(s1_size, align_of::<S>()).unwrap();
    let s1_ptr = unsafe { alloc(s1_layout) };
    if s1_ptr.is_null() {
        std::process::exit(1);
    }

    // Allocate memory for s2 (5 doubles)
    let s2_size = size_of::<S>() + size_of::<f64>() * 5;
    let s2_layout = Layout::from_size_align(s2_size, align_of::<S>()).unwrap();
    let s2_ptr = unsafe { alloc(s2_layout) };
    if s2_ptr.is_null() {
        unsafe { dealloc(s1_ptr, s1_layout) };
        std::process::exit(1);
    }

    // Cast to pointers to S
    let s1 = s1_ptr as *mut S;
    let s2 = s2_ptr as *mut S;

    // Access the flexible array member through pointer arithmetic
    unsafe {
        let s1_d_ptr = (s1 as *mut u8).add(size_of::<S>()) as *mut f64;
        *s1_d_ptr = 42.0;

        let s2_d_ptr = (s2 as *mut u8).add(size_of::<S>()) as *mut f64;
        *s2_d_ptr = 24.0;

        // Verify values
        if *s1_d_ptr != 42.0 || *s2_d_ptr != 24.0 {
            dealloc(s1_ptr, s1_layout);
            dealloc(s2_ptr, s2_layout);
            std::process::exit(1);
        }
    }

    // Clean up
    unsafe {
        dealloc(s1_ptr, s1_layout);
        dealloc(s2_ptr, s2_layout);
    }

    std::process::exit(0);
}