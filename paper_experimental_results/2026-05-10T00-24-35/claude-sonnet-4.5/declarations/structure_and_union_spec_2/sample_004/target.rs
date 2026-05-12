use std::alloc::{alloc, dealloc, Layout};
use std::mem::{align_of, size_of};
use std::ptr;

fn main() {
    #[repr(C)]
    struct S {
        n: i32,
    }

    #[repr(C)]
    struct SS {
        n: i32,
    }

    if size_of::<S>() < size_of::<SS>() {
        std::process::exit(1);
    }

    let offset_d = size_of::<S>();
    if offset_d != size_of::<S>() {
        std::process::exit(1);
    }

    let s1_size = size_of::<S>() + size_of::<f64>() * 8;
    let s2_size = size_of::<S>() + size_of::<f64>() * 5;

    let s1_layout = Layout::from_size_align(s1_size, align_of::<S>().max(align_of::<f64>())).unwrap();
    let s2_layout = Layout::from_size_align(s2_size, align_of::<S>().max(align_of::<f64>())).unwrap();

    let s1_ptr = unsafe { alloc(s1_layout) };
    let s2_ptr = unsafe { alloc(s2_layout) };

    if s1_ptr.is_null() || s2_ptr.is_null() {
        if !s1_ptr.is_null() {
            unsafe { dealloc(s1_ptr, s1_layout) };
        }
        if !s2_ptr.is_null() {
            unsafe { dealloc(s2_ptr, s2_layout) };
        }
        std::process::exit(1);
    }

    let d1_ptr = unsafe { s1_ptr.add(size_of::<S>()) as *mut f64 };
    let d2_ptr = unsafe { s2_ptr.add(size_of::<S>()) as *mut f64 };

    unsafe {
        ptr::write(d1_ptr, 42.0);
        ptr::write(d2_ptr, 24.0);
    }

    let val1 = unsafe { ptr::read(d1_ptr) };
    let val2 = unsafe { ptr::read(d2_ptr) };

    if val1 != 42.0 || val2 != 24.0 {
        unsafe {
            dealloc(s1_ptr, s1_layout);
            dealloc(s2_ptr, s2_layout);
        }
        std::process::exit(1);
    }

    unsafe {
        dealloc(s1_ptr, s1_layout);
        dealloc(s2_ptr, s2_layout);
    }

    std::process::exit(0);
}