use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, align_of};
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

    let size_s1 = size_of::<S>() + size_of::<f64>() * 8;
    let size_s2 = size_of::<S>() + size_of::<f64>() * 5;

    let layout_s1 = Layout::from_size_align(size_s1, align_of::<S>()).unwrap();
    let layout_s2 = Layout::from_size_align(size_s2, align_of::<S>()).unwrap();

    let s1 = unsafe { alloc(layout_s1) as *mut S };
    let s2 = unsafe { alloc(layout_s2) as *mut S };

    if s1.is_null() || s2.is_null() {
        if !s1.is_null() {
            unsafe { dealloc(s1 as *mut u8, layout_s1) };
        }
        if !s2.is_null() {
            unsafe { dealloc(s2 as *mut u8, layout_s2) };
        }
        std::process::exit(1);
    }

    unsafe {
        let d1_ptr = (s1 as *mut u8).add(size_of::<S>()) as *mut f64;
        let d2_ptr = (s2 as *mut u8).add(size_of::<S>()) as *mut f64;

        ptr::write(d1_ptr, 42.0);
        ptr::write(d2_ptr, 24.0);

        if ptr::read(d1_ptr) != 42.0 || ptr::read(d2_ptr) != 24.0 {
            dealloc(s1 as *mut u8, layout_s1);
            dealloc(s2 as *mut u8, layout_s2);
            std::process::exit(1);
        }

        dealloc(s1 as *mut u8, layout_s1);
        dealloc(s2 as *mut u8, layout_s2);
    }

    std::process::exit(0);
}