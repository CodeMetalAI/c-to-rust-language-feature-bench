use std::alloc::{alloc, dealloc, Layout};
use std::mem::{align_of, offset_of, size_of};
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

    let size1 = size_of::<S>() + size_of::<f64>() * 8;
    let size2 = size_of::<S>() + size_of::<f64>() * 5;

    let layout1 = Layout::from_size_align(size1, align_of::<S>()).unwrap();
    let layout2 = Layout::from_size_align(size2, align_of::<S>()).unwrap();

    let s1 = unsafe { alloc(layout1) as *mut S };
    let s2 = unsafe { alloc(layout2) as *mut S };

    if s1.is_null() || s2.is_null() {
        std::process::exit(1);
    }

    unsafe {
        let d1_ptr = (s1 as *mut u8).add(size_of::<S>()) as *mut f64;
        let d2_ptr = (s2 as *mut u8).add(size_of::<S>()) as *mut f64;

        ptr::write(d1_ptr, 42.0);
        ptr::write(d2_ptr, 24.0);

        if ptr::read(d1_ptr) != 42.0 || ptr::read(d2_ptr) != 24.0 {
            dealloc(s1 as *mut u8, layout1);
            dealloc(s2 as *mut u8, layout2);
            std::process::exit(1);
        }

        dealloc(s1 as *mut u8, layout1);
        dealloc(s2 as *mut u8, layout2);
    }

    std::process::exit(0);
}