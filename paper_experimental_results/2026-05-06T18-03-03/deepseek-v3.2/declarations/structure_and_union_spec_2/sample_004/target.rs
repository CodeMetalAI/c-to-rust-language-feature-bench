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
    let d_offset = size_of::<i32>();
    if d_offset != size_of::<S>() {
        std::process::exit(1);
    }

    // Allocate memory for S with 8 additional f64s
    let layout1 = Layout::from_size_align(
        size_of::<S>() + size_of::<f64>() * 8,
        align_of::<S>(),
    ).unwrap();
    let s1_ptr = unsafe { alloc(layout1) as *mut S };
    if s1_ptr.is_null() {
        std::process::exit(1);
    }

    // Allocate memory for S with 5 additional f64s
    let layout2 = Layout::from_size_align(
        size_of::<S>() + size_of::<f64>() * 5,
        align_of::<S>(),
    ).unwrap();
    let s2_ptr = unsafe { alloc(layout2) as *mut S };
    if s2_ptr.is_null() {
        unsafe { dealloc(s1_ptr as *mut u8, layout1) };
        std::process::exit(1);
    }

    // Initialize the flexible array members
    unsafe {
        let d1_ptr = (*s1_ptr).d.as_ptr() as *mut f64;
        *d1_ptr = 42.0;

        let d2_ptr = (*s2_ptr).d.as_ptr() as *mut f64;
        *d2_ptr = 24.0;

        // Check values
        if *d1_ptr != 42.0 || *d2_ptr != 24.0 {
            dealloc(s1_ptr as *mut u8, layout1);
            dealloc(s2_ptr as *mut u8, layout2);
            std::process::exit(1);
        }

        // Free memory
        dealloc(s1_ptr as *mut u8, layout1);
        dealloc(s2_ptr as *mut u8, layout2);
    }

    std::process::exit(0);
}