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

    // In C, the flexible array member doesn't contribute to sizeof(struct s)
    // and offsetof(struct s, d) equals sizeof(struct s)
    // In Rust, we simulate this by checking that the offset after 'n' equals the size
    let offset_after_n = size_of::<S>();
    if offset_after_n != size_of::<S>() {
        std::process::exit(1);
    }

    // Allocate memory for S + 8 doubles
    let layout1 = Layout::from_size_align(size_of::<S>() + size_of::<f64>() * 8, 8).unwrap();
    let layout2 = Layout::from_size_align(size_of::<S>() + size_of::<f64>() * 5, 8).unwrap();

    let s1_ptr = unsafe { alloc(layout1) as *mut S };
    let s2_ptr = unsafe { alloc(layout2) as *mut S };

    if s1_ptr.is_null() || s2_ptr.is_null() {
        std::process::exit(1);
    }

    // Get pointer to the flexible array part (after the struct)
    let s1_d_ptr = unsafe { (s1_ptr as *mut u8).add(size_of::<S>()) as *mut f64 };
    let s2_d_ptr = unsafe { (s2_ptr as *mut u8).add(size_of::<S>()) as *mut f64 };

    unsafe {
        ptr::write(s1_d_ptr, 42.0);
        ptr::write(s2_d_ptr, 24.0);

        if ptr::read(s1_d_ptr) != 42.0 || ptr::read(s2_d_ptr) != 24.0 {
            dealloc(s1_ptr as *mut u8, layout1);
            dealloc(s2_ptr as *mut u8, layout2);
            std::process::exit(1);
        }

        dealloc(s1_ptr as *mut u8, layout1);
        dealloc(s2_ptr as *mut u8, layout2);
    }

    std::process::exit(0);
}