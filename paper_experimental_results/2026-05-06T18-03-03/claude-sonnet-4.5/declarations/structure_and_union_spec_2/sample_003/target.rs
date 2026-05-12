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

    let layout1 = Layout::from_size_align(s1_size, align_of::<S>().max(align_of::<f64>())).unwrap();
    let layout2 = Layout::from_size_align(s2_size, align_of::<S>().max(align_of::<f64>())).unwrap();

    let s1_ptr = unsafe { alloc(layout1) };
    let s2_ptr = unsafe { alloc(layout2) };

    if s1_ptr.is_null() || s2_ptr.is_null() {
        if !s1_ptr.is_null() {
            unsafe { dealloc(s1_ptr, layout1) };
        }
        if !s2_ptr.is_null() {
            unsafe { dealloc(s2_ptr, layout2) };
        }
        std::process::exit(1);
    }

    unsafe {
        let d1_ptr = s1_ptr.add(size_of::<S>()) as *mut f64;
        let d2_ptr = s2_ptr.add(size_of::<S>()) as *mut f64;

        ptr::write(d1_ptr, 42.0);
        ptr::write(d2_ptr, 24.0);

        if ptr::read(d1_ptr) != 42.0 || ptr::read(d2_ptr) != 24.0 {
            dealloc(s1_ptr, layout1);
            dealloc(s2_ptr, layout2);
            std::process::exit(1);
        }

        dealloc(s1_ptr, layout1);
        dealloc(s2_ptr, layout2);
    }

    std::process::exit(0);
}