use std::alloc::{alloc, dealloc, Layout};
use std::mem::{align_of, size_of};
use std::ptr;

fn main() {
    #[repr(C)]
    struct S {
        n: i32,
    }

    #[repr(C)]
    struct Ss {
        n: i32,
    }

    if size_of::<S>() < size_of::<Ss>() {
        std::process::exit(1);
    }

    let offset_d = size_of::<S>();
    if offset_d != size_of::<S>() {
        std::process::exit(1);
    }

    let s1_layout = Layout::from_size_align(
        size_of::<S>() + size_of::<f64>() * 8,
        align_of::<S>().max(align_of::<f64>()),
    )
    .unwrap();
    
    let s2_layout = Layout::from_size_align(
        size_of::<S>() + size_of::<f64>() * 5,
        align_of::<S>().max(align_of::<f64>()),
    )
    .unwrap();

    let s1_ptr = unsafe { alloc(s1_layout) };
    let s2_ptr = unsafe { alloc(s2_layout) };

    if s1_ptr.is_null() || s2_ptr.is_null() {
        std::process::exit(1);
    }

    unsafe {
        let s1_d_ptr = s1_ptr.add(size_of::<S>()) as *mut f64;
        let s2_d_ptr = s2_ptr.add(size_of::<S>()) as *mut f64;

        ptr::write(s1_d_ptr, 42.0);
        ptr::write(s2_d_ptr, 24.0);

        if ptr::read(s1_d_ptr) != 42.0 || ptr::read(s2_d_ptr) != 24.0 {
            dealloc(s1_ptr, s1_layout);
            dealloc(s2_ptr, s2_layout);
            std::process::exit(1);
        }

        dealloc(s1_ptr, s1_layout);
        dealloc(s2_ptr, s2_layout);
    }

    std::process::exit(0);
}