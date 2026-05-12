use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, offset_of};
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

    let s_size = size_of::<S>();
    let d_offset = s_size;

    if d_offset != s_size {
        std::process::exit(1);
    }

    let s1_size = s_size + size_of::<f64>() * 8;
    let s2_size = s_size + size_of::<f64>() * 5;

    let layout1 = Layout::from_size_align(s1_size, std::mem::align_of::<S>()).unwrap();
    let layout2 = Layout::from_size_align(s2_size, std::mem::align_of::<S>()).unwrap();

    let s1_ptr = unsafe { alloc(layout1) as *mut S };
    let s2_ptr = unsafe { alloc(layout2) as *mut S };

    if s1_ptr.is_null() || s2_ptr.is_null() {
        std::process::exit(1);
    }

    unsafe {
        let d1_ptr = (s1_ptr as *mut u8).add(d_offset) as *mut f64;
        let d2_ptr = (s2_ptr as *mut u8).add(d_offset) as *mut f64;

        ptr::write(d1_ptr, 42.0);
        ptr::write(d2_ptr, 24.0);

        if ptr::read(d1_ptr) != 42.0 || ptr::read(d2_ptr) != 24.0 {
            dealloc(s1_ptr as *mut u8, layout1);
            dealloc(s2_ptr as *mut u8, layout2);
            std::process::exit(1);
        }

        dealloc(s1_ptr as *mut u8, layout1);
        dealloc(s2_ptr as *mut u8, layout2);
    }
}