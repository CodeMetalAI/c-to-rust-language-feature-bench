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
    // Check that S is at least as large as Ss
    if size_of::<S>() < size_of::<Ss>() {
        std::process::exit(1);
    }

    // Check that d is at the end of S (no padding after n)
    let offset_d = size_of::<i32>();
    if offset_d != size_of::<S>() {
        std::process::exit(1);
    }

    // Allocate memory for S with 8 extra doubles
    let layout_s1 = Layout::from_size_align(
        size_of::<S>() + size_of::<f64>() * 8,
        align_of::<S>(),
    ).unwrap();
    let s1_ptr = unsafe { alloc(layout_s1) } as *mut S;
    if s1_ptr.is_null() {
        std::process::exit(1);
    }

    // Allocate memory for S with 5 extra doubles
    let layout_s2 = Layout::from_size_align(
        size_of::<S>() + size_of::<f64>() * 5,
        align_of::<S>(),
    ).unwrap();
    let s2_ptr = unsafe { alloc(layout_s2) } as *mut S;
    if s2_ptr.is_null() {
        unsafe { dealloc(s1_ptr as *mut u8, layout_s1) };
        std::process::exit(1);
    }

    unsafe {
        // Initialize the flexible array part
        let s1_d_ptr = (*s1_ptr).d.as_ptr() as *mut f64;
        s1_d_ptr.write(42.0);

        let s2_d_ptr = (*s2_ptr).d.as_ptr() as *mut f64;
        s2_d_ptr.write(24.0);

        // Check the values
        if s1_d_ptr.read() != 42.0 || s2_d_ptr.read() != 24.0 {
            dealloc(s1_ptr as *mut u8, layout_s1);
            dealloc(s2_ptr as *mut u8, layout_s2);
            std::process::exit(1);
        }

        // Clean up
        dealloc(s1_ptr as *mut u8, layout_s1);
        dealloc(s2_ptr as *mut u8, layout_s2);
    }

    std::process::exit(0);
}